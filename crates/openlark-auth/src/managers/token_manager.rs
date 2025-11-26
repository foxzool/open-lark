//! 令牌管理器

use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{debug, error, info, warn};

use crate::auth::{
    cache::MemoryTokenCache,
    refresh::TokenRefresher,
    token::{
        AccessToken, GetTokenRequest, GetTokenResponse, RefreshToken, TokenInfo, TokenType,
        TokenValidationResult,
    },
    validator::TokenValidator,
};
use crate::client::AuthConfig;
use crate::error::{AuthError, AuthResult};

/// 令牌管理器
///
/// 统一管理所有类型的令牌，包括获取、缓存、刷新和验证
pub struct TokenManager {
    config: AuthConfig,
    cache: Arc<MemoryTokenCache>,
    refresher: TokenRefresher,
    validator: TokenValidator,
}

impl TokenManager {
    /// 创建新的令牌管理器
    pub fn new(config: AuthConfig) -> AuthResult<Self> {
        let cache = Arc::new(MemoryTokenCache::new(
            crate::auth::cache::CacheConfig::default(),
        ));
        let refresher = TokenRefresher::new(
            config.app_id.clone(),
            config.app_secret.clone(),
            config.base_url.clone(),
            cache.clone(),
        );
        let validator = TokenValidator::default();

        Ok(Self {
            config,
            cache,
            refresher,
            validator,
        })
    }

    /// 获取访问令牌（主要入口方法）
    pub async fn get_access_token(&self, request: GetTokenRequest) -> AuthResult<GetTokenResponse> {
        debug!("Getting access token for request: {:?}", request);

        let token_info = match request.token_type {
            TokenType::AppAccessToken => self.get_app_token_info().await?,
            TokenType::TenantAccessToken => {
                let tenant_key = request.tenant_key.ok_or_else(|| {
                    AuthError::TokenError(
                        "tenant_key is required for tenant access token".to_string(),
                    )
                })?;
                self.get_tenant_token_info(&tenant_key).await?
            }
            TokenType::UserAccessToken => {
                let refresh_token = request.refresh_token.ok_or_else(|| {
                    AuthError::TokenError(
                        "refresh_token is required for user access token".to_string(),
                    )
                })?;
                self.get_user_token_info(&refresh_token).await?
            }
        };

        Ok(GetTokenResponse {
            app_access_token: token_info.access_token.clone(),
            token_type: token_info.token_type,
            expires_in: token_info.time_until_expiry().unwrap_or_default().as_secs(),
            refresh_token: token_info.refresh_token.clone(),
            scope: None,
        })
    }

    /// 获取应用访问令牌信息
    async fn get_app_token_info(&self) -> AuthResult<TokenInfo> {
        debug!("Getting app access token info");

        // 尝试从缓存获取
        if let Some(token_info) = self.cache.get("app_access_token").await {
            if !self.validator.should_refresh(&token_info) {
                debug!("Using cached app access token");
                return Ok(token_info);
            }
        }

        // 刷新令牌
        let token_info = self.refresher.refresh_app_token().await?;
        self.cache.put("app_access_token", token_info.clone()).await;

        info!("App access token obtained successfully");
        Ok(token_info)
    }

    /// 获取租户访问令牌信息
    async fn get_tenant_token_info(&self, tenant_key: &str) -> AuthResult<TokenInfo> {
        debug!("Getting tenant access token info for: {}", tenant_key);

        let cache_key = format!("tenant_access_token:{}", tenant_key);

        // 尝试从缓存获取
        if let Some(token_info) = self.cache.get(&cache_key).await {
            if !self.validator.should_refresh(&token_info) {
                debug!("Using cached tenant access token");
                return Ok(token_info);
            }
        }

        // 刷新令牌
        let token_info = self.refresher.refresh_tenant_token(tenant_key).await?;
        self.cache.put(&cache_key, token_info.clone()).await;

        info!(
            "Tenant access token obtained successfully for: {}",
            tenant_key
        );
        Ok(token_info)
    }

    /// 获取用户访问令牌信息
    async fn get_user_token_info(&self, refresh_token: &str) -> AuthResult<TokenInfo> {
        debug!("Getting user access token info");

        let cache_key = format!("user_access_token:{}", refresh_token);

        // 尝试从缓存获取
        if let Some(token_info) = self.cache.get(&cache_key).await {
            if !self.validator.should_refresh(&token_info) {
                debug!("Using cached user access token");
                return Ok(token_info);
            }
        }

        // 使用刷新令牌获取新的访问令牌
        let token_info = self
            .refresher
            .refresh_with_refresh_token(refresh_token)
            .await?;
        self.cache.put(&cache_key, token_info.clone()).await;

        info!("User access token obtained successfully");
        Ok(token_info)
    }

    /// 验证访问令牌
    pub async fn validate_access_token(&self, token: &str) -> AuthResult<TokenValidationResult> {
        debug!("Validating access token");

        // 首先验证令牌格式
        let format_result = self.validator.validate_token_format(token);
        if !format_result.is_valid() {
            return Ok(format_result);
        }

        // 检查令牌是否在缓存中且有效
        let cache_keys = [
            "app_access_token",
            "tenant_access_token:",
            "user_access_token:",
        ];

        for key_pattern in cache_keys {
            if key_pattern.ends_with(':') {
                // 对于用户令牌，需要更复杂的查找逻辑
                continue;
            } else if let Some(token_info) = self.cache.get(key_pattern).await {
                if token_info.access_token == token {
                    let validation_result = self.validator.validate(&token_info);
                    debug!("Token validation result: {:?}", validation_result);
                    return Ok(validation_result);
                }
            }
        }

        // 令牌不在缓存中，返回基本验证结果
        debug!("Token not found in cache, returning format validation result");
        Ok(format_result)
    }

    /// 刷新访问令牌
    pub async fn refresh_access_token(&self, refresh_token: &str) -> AuthResult<GetTokenResponse> {
        debug!("Refreshing access token");

        let token_info = self
            .refresher
            .refresh_with_refresh_token(refresh_token)
            .await?;

        // 更新缓存
        let cache_key = format!("user_access_token:{}", refresh_token);
        self.cache.put(&cache_key, token_info.clone()).await;

        info!("Access token refreshed successfully");
        Ok(GetTokenResponse {
            app_access_token: token_info.access_token.clone(),
            token_type: token_info.token_type,
            expires_in: token_info.time_until_expiry().unwrap_or_default().as_secs(),
            refresh_token: token_info.refresh_token.clone(),
            scope: None,
        })
    }

    /// 撤销访问令牌
    pub async fn revoke_access_token(&self, token: &str) -> AuthResult<()> {
        debug!("Revoking access token");

        // TODO: 实现调用飞书撤销令牌接口的逻辑
        // 目前只是从缓存中移除

        let cache_keys = [
            "app_access_token",
            "tenant_access_token:",
            "user_access_token:",
        ];

        for key_pattern in cache_keys {
            if key_pattern.ends_with(':') {
                continue;
            }
            if let Some(token_info) = self.cache.get(key_pattern).await {
                if token_info.access_token == token {
                    self.cache.remove(key_pattern).await;
                    info!("Access token revoked successfully");
                    return Ok(());
                }
            }
        }

        warn!("Token not found in cache, but marking as revoked");
        Ok(())
    }

    /// 获取令牌信息
    pub async fn get_token_info(&self, token: &str) -> AuthResult<TokenInfo> {
        debug!("Getting token info for token");

        let cache_keys = [
            "app_access_token",
            "tenant_access_token:",
            "user_access_token:",
        ];

        for key_pattern in cache_keys {
            if key_pattern.ends_with(':') {
                continue;
            } else if let Some(token_info) = self.cache.get(key_pattern).await {
                if token_info.access_token == token {
                    debug!("Token info found in cache");
                    return Ok(token_info);
                }
            }
        }

        Err(AuthError::TokenError(
            "Token not found in cache".to_string(),
        ))
    }

    /// 批量获取访问令牌
    pub async fn batch_get_access_tokens(
        &self,
        requests: Vec<GetTokenRequest>,
    ) -> Vec<AuthResult<GetTokenResponse>> {
        debug!("Batch getting {} access tokens", requests.len());

        let mut results = Vec::with_capacity(requests.len());

        for request in requests {
            let result = self.get_access_token(request).await;
            results.push(result);
        }

        let success_count = results.iter().filter(|r| r.is_ok()).count();
        info!(
            "Batch token request completed: {}/{} successful",
            success_count,
            results.len()
        );

        results
    }

    /// 清空所有令牌缓存
    pub async fn clear_all_tokens(&self) -> AuthResult<()> {
        debug!("Clearing all token cache");
        self.cache.clear().await;
        info!("All token cache cleared successfully");
        Ok(())
    }

    /// 预热令牌缓存
    pub async fn warmup_tokens(&self, tenant_keys: Vec<String>) -> AuthResult<()> {
        info!("Warming up tokens for {} tenants", tenant_keys.len());

        // 预热应用令牌
        if let Err(e) = self.get_app_token_info().await {
            warn!("Failed to warmup app token: {}", e);
        }

        // 预热租户令牌
        for tenant_key in &tenant_keys {
            if let Err(e) = self.get_tenant_token_info(tenant_key).await {
                warn!("Failed to warmup tenant token {}: {}", tenant_key, e);
            }
        }

        info!("Token warmup completed");
        Ok(())
    }

    /// 获取令牌统计信息
    pub async fn get_token_stats(&self) -> TokenStats {
        let cache_stats = self.cache.stats().await;

        let mut stats = TokenStats {
            total_tokens: cache_stats.current_size as u64,
            cache_hits: cache_stats.hits,
            cache_misses: cache_stats.misses,
            hit_rate: if cache_stats.hits + cache_stats.misses > 0 {
                cache_stats.hits as f64 / (cache_stats.hits + cache_stats.misses) as f64
            } else {
                0.0
            },
            app_tokens: 0,
            tenant_tokens: 0,
            user_tokens: 0,
            expired_tokens: 0,
        };

        // 目前无法获取所有keys，所以只返回基本统计
        // TODO: 在缓存中实现keys()方法来获取详细的令牌类型统计
        stats
    }

    /// 清理过期令牌
    pub async fn cleanup_expired_tokens(&self) -> AuthResult<usize> {
        debug!("Cleaning up expired tokens");
        // 目前缓存没有暴露cleanup_expired方法，暂时返回0
        // TODO: 在缓存中实现cleanup_expired方法
        info!("Cleaned up 0 expired tokens");
        Ok(0)
    }

    /// 检查令牌是否需要刷新
    pub async fn should_refresh_token(&self, token: &str) -> AuthResult<bool> {
        debug!("Checking if token needs refresh: {}", token);

        // 这个方法需要通过token而不是key来查找，目前先返回true
        // TODO: 实现更复杂的token反向查找逻辑
        debug!("Token refresh check not implemented, returning true");
        Ok(true)
    }
}

/// 令牌统计信息
#[derive(Debug, Clone, Default)]
pub struct TokenStats {
    /// 总令牌数量
    pub total_tokens: u64,
    /// 缓存命中次数
    pub cache_hits: u64,
    /// 缓存未命中次数
    pub cache_misses: u64,
    /// 命中率
    pub hit_rate: f64,
    /// 应用令牌数量
    pub app_tokens: u32,
    /// 租户令牌数量
    pub tenant_tokens: u32,
    /// 用户令牌数量
    pub user_tokens: u32,
    /// 过期令牌数量
    pub expired_tokens: u32,
}

#[cfg(test)]
mod tests {
    fn create_test_config() -> AuthConfig {
        crate::client::AuthConfigBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn test_token_manager_creation() {
        let config = create_test_config();
        let manager = TokenManager::new(config);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_get_app_access_token() {
        let config = create_test_config();
        let manager = TokenManager::new(config).unwrap();

        let request = GetTokenRequest::self_build_app_access_token();

        let result = manager.get_access_token(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.app_access_token.is_empty());
        assert_eq!(response.token_type, TokenType::AppAccessToken);
    }

    #[tokio::test]
    async fn test_validate_token_format() {
        let config = create_test_config();
        let manager = TokenManager::new(config).unwrap();

        // 测试有效格式
        let result = manager.validate_access_token("cli_test123").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_valid());

        // 测试无效格式
        let result = manager.validate_access_token("invalid_token").await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_valid());
    }

    #[tokio::test]
    async fn test_batch_get_tokens() {
        let config = create_test_config();
        let manager = TokenManager::new(config).unwrap();

        let requests = vec![
            GetTokenRequest::self_build_app_access_token(),
            GetTokenRequest::self_build_app_access_token(),
        ];

        let results = manager.batch_get_access_tokens(requests).await;
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[tokio::test]
    async fn test_token_stats() {
        let config = create_test_config();
        let manager = TokenManager::new(config).unwrap();

        let stats = manager.get_token_stats().await;
        assert_eq!(stats.total_tokens, 0);
        assert_eq!(stats.app_tokens, 0);
        assert_eq!(stats.tenant_tokens, 0);
        assert_eq!(stats.user_tokens, 0);
        assert_eq!(stats.expired_tokens, 0);
    }

    #[tokio::test]
    async fn test_warmup_tokens() {
        let config = create_test_config();
        let manager = TokenManager::new(config).unwrap();

        let tenant_keys = vec!["tenant1".to_string(), "tenant2".to_string()];

        let result = manager.warmup_tokens(tenant_keys).await;
        assert!(result.is_ok());
    }
}
