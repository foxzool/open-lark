//! 令牌客户端

use std::sync::Arc;
use tracing::{debug, info, warn};

use super::config::AuthConfig;
use crate::auth::{
    cache::MemoryTokenCache,
    refresh::TokenRefresher,
    token::{AccessToken, GetTokenRequest, GetTokenResponse, RefreshToken, TokenInfo, TokenType},
    validator::TokenValidator,
};
use crate::error::{AuthError, AuthResult};

/// 令牌客户端
///
/// 专门处理各种令牌的获取、刷新和管理操作
#[derive(Debug, Clone)]
pub struct TokenClient {
    config: AuthConfig,
    cache: Arc<MemoryTokenCache>,
    refresher: TokenRefresher,
    validator: TokenValidator,
}

impl TokenClient {
    /// 创建新的令牌客户端
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

    /// 获取访问令牌（通用方法）
    pub async fn get_access_token(&self, request: GetTokenRequest) -> AuthResult<AccessToken> {
        debug!("Getting access token for request: {:?}", request);

        match request.token_type {
            TokenType::AppAccessToken => self.get_app_access_token().await,
            TokenType::TenantAccessToken => {
                let tenant_key = request.tenant_key.ok_or_else(|| {
                    AuthError::TokenError(
                        "tenant_key is required for tenant access token".to_string(),
                    )
                })?;
                self.get_tenant_access_token(&tenant_key).await
            }
            TokenType::UserAccessToken => {
                let refresh_token = request.refresh_token.ok_or_else(|| {
                    AuthError::TokenError(
                        "refresh_token is required for user access token".to_string(),
                    )
                })?;
                self.get_user_access_token(&refresh_token).await
            }
        }
    }

    /// 获取应用访问令牌
    pub async fn get_app_access_token(&self) -> AuthResult<AccessToken> {
        debug!("Getting app access token");

        // 尝试从缓存获取
        if let Some(token_info) = self.cache.get("app_access_token").await {
            if !self.validator.should_refresh(&token_info) {
                debug!("Using cached app access token");
                return Ok(AccessToken::new(token_info.access_token));
            }
        }

        // 刷新令牌
        let token_info = self.refresher.refresh_app_token().await?;

        // 缓存新令牌
        self.cache.put("app_access_token", token_info.clone()).await;

        info!("Successfully obtained new app access token");
        Ok(AccessToken::new(token_info.access_token))
    }

    /// 获取租户访问令牌
    pub async fn get_tenant_access_token(&self, tenant_key: &str) -> AuthResult<AccessToken> {
        debug!("Getting tenant access token for tenant: {}", tenant_key);

        let cache_key = format!("tenant_access_token:{}", tenant_key);

        // 尝试从缓存获取
        if let Some(token_info) = self.cache.get(&cache_key).await {
            if !self.validator.should_refresh(&token_info) {
                debug!("Using cached tenant access token");
                return Ok(AccessToken::new(token_info.access_token));
            }
        }

        // 刷新令牌
        let token_info = self.refresher.refresh_tenant_token(tenant_key).await?;

        // 缓存新令牌
        self.cache.put(&cache_key, token_info.clone()).await;

        info!(
            "Successfully obtained new tenant access token for tenant: {}",
            tenant_key
        );
        Ok(AccessToken::new(token_info.access_token))
    }

    /// 获取用户访问令牌
    pub async fn get_user_access_token(&self, refresh_token: &str) -> AuthResult<AccessToken> {
        debug!("Getting user access token with refresh token");

        // 用户访问令牌通常有较长的有效期，这里使用刷新令牌来获取新的访问令牌
        let token_info = self
            .refresher
            .refresh_with_refresh_token(refresh_token)
            .await?;

        // 缓存用户令牌（较短的TTL）
        let cache_key = format!("user_access_token:{}", refresh_token);
        self.cache.put(&cache_key, token_info.clone()).await;

        info!("Successfully obtained new user access token");
        Ok(AccessToken::new(token_info.access_token))
    }

    /// 刷新访问令牌
    pub async fn refresh_access_token(&self, refresh_token: &str) -> AuthResult<AccessToken> {
        debug!("Refreshing access token with refresh token");

        let token_info = self
            .refresher
            .refresh_with_refresh_token(refresh_token)
            .await?;

        // 更新缓存
        let cache_key = format!("user_access_token:{}", refresh_token);
        self.cache.put(&cache_key, token_info.clone()).await;

        info!("Successfully refreshed access token");
        Ok(AccessToken::new(token_info.access_token))
    }

    /// 验证访问令牌
    pub async fn validate_access_token(
        &self,
        token: &str,
    ) -> AuthResult<crate::auth::token::TokenValidationResult> {
        debug!("Validating access token");

        // 首先验证令牌格式
        let format_result = self.validator.validate_token_format(token);
        if !format_result.is_valid() {
            return Ok(format_result);
        }

        // TODO: 可以添加调用飞书令牌验证接口的逻辑
        // 目前返回基本验证结果
        Ok(format_result)
    }

    /// 获取令牌信息
    pub async fn get_token_info(&self, token: &str) -> AuthResult<TokenInfo> {
        debug!("Getting token info for token");

        // 尝试从缓存中查找令牌信息
        let cache_keys = [
            "app_access_token",
            "user_access_token:",
            "tenant_access_token:",
        ];

        for key_pattern in cache_keys {
            if key_pattern.ends_with(':') {
                // 对于用户令牌，需要更复杂的查找逻辑
                continue;
            } else if let Some(token_info) = self.cache.get(key_pattern).await {
                if token_info.access_token == token {
                    debug!("Found token info in cache");
                    return Ok(token_info);
                }
            }
        }

        // 如果缓存中没有找到，返回错误
        Err(AuthError::TokenError(
            "Token not found in cache".to_string(),
        ))
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
                    info!("Successfully revoked access token");
                    return Ok(());
                }
            }
        }

        warn!("Token not found in cache, but marking as revoked");
        Ok(())
    }

    /// 批量获取令牌
    pub async fn batch_get_access_tokens(
        &self,
        requests: Vec<GetTokenRequest>,
    ) -> Vec<AuthResult<AccessToken>> {
        debug!("Batch getting {} access tokens", requests.len());

        let mut results = Vec::with_capacity(requests.len());

        for request in requests {
            let result = self.get_access_token(request).await;
            results.push(result);
        }

        info!(
            "Batch token request completed: {} successes, {} failures",
            results.iter().filter(|r| r.is_ok()).count(),
            results.iter().filter(|r| r.is_err()).count()
        );

        results
    }

    /// 获取客户端统计信息
    pub async fn get_stats(&self) -> crate::auth::cache::CacheStats {
        self.cache.stats().await
    }

    /// 清空所有令牌缓存
    pub async fn clear_cache(&self) -> AuthResult<()> {
        debug!("Clearing all token cache");
        self.cache.clear().await;
        info!("Token cache cleared successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::client::config::AuthConfigBuilder;
    use crate::auth::token::{AppType, GetTokenRequest};

    fn create_test_config() -> AuthConfig {
        AuthConfigBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn test_token_client_creation() {
        let config = create_test_config();
        let client = TokenClient::new(config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_get_token_request() {
        let config = create_test_config();
        let client = TokenClient::new(config).unwrap();

        let request = GetTokenRequest::self_build_app_access_token();

        // 由于这是模拟实现，应该能正常工作
        let result = client.get_access_token(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_get_tokens() {
        let config = create_test_config();
        let client = TokenClient::new(config).unwrap();

        let requests = vec![
            GetTokenRequest::self_build_app_access_token(),
            GetTokenRequest::self_build_app_access_token(),
        ];

        let results = client.batch_get_access_tokens(requests).await;
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[tokio::test]
    async fn test_app_type_distinction() {
        let config = create_test_config();
        let client = TokenClient::new(config).unwrap();

        // 测试自建应用访问令牌请求
        let self_build_request = GetTokenRequest::self_build_app_access_token();
        assert_eq!(self_build_request.app_type, AppType::SelfBuild);
        assert_eq!(self_build_request.token_type, TokenType::AppAccessToken);

        // 测试商店应用访问令牌请求
        let store_request = GetTokenRequest::store_app_access_token();
        assert_eq!(store_request.app_type, AppType::Store);
        assert_eq!(store_request.token_type, TokenType::AppAccessToken);

        // 测试租户访问令牌请求
        let tenant_key = "test_tenant".to_string();
        let self_build_tenant_request = GetTokenRequest::self_build_tenant_access_token(tenant_key.clone());
        assert_eq!(self_build_tenant_request.app_type, AppType::SelfBuild);
        assert_eq!(self_build_tenant_request.token_type, TokenType::TenantAccessToken);
        assert_eq!(self_build_tenant_request.tenant_key, Some(tenant_key));

        // 测试带作用域的请求
        let scoped_request = GetTokenRequest::self_build_app_access_token()
            .with_scope("contact:base".to_string());
        assert_eq!(scoped_request.scope, Some("contact:base".to_string()));
    }

    #[tokio::test]
    async fn test_token_validation() {
        let config = create_test_config();
        let client = TokenClient::new(config).unwrap();

        // 测试有效格式
        let result = client.validate_access_token("cli_test123").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_valid());

        // 测试无效格式
        let result = client.validate_access_token("invalid_token").await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_valid());
    }
}
