//! 令牌刷新实现
//!
//! 此模块提供了令牌刷新机制的基础框架，支持多种令牌类型的刷新。

use std::time::{Duration, SystemTime};
use tracing::{debug, error, info, warn};

use super::cache::MemoryTokenCache;
use super::token::{TokenInfo, TokenType};
use crate::error::{AuthError, AuthResult};

/// 刷新令牌响应
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RefreshTokenResponse {
    /// 应用访问令牌
    #[serde(rename = "app_access_token")]
    pub app_access_token: Option<String>,
    /// 租户访问令牌
    #[serde(rename = "tenant_access_token")]
    pub tenant_access_token: Option<String>,
    /// 刷新令牌
    #[serde(rename = "refresh_token")]
    pub refresh_token: Option<String>,
    /// 过期时间（秒）
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
    /// 令牌类型
    #[serde(rename = "token_type")]
    pub token_type: Option<String>,
}

/// 令牌刷新器
pub struct TokenRefresher {
    app_id: String,
    app_secret: String,
    base_url: String,
    cache: std::sync::Arc<MemoryTokenCache>,
    refresh_config: super::token::TokenRefreshConfig,
}

impl TokenRefresher {
    /// 创建新的令牌刷新器
    pub fn new(
        app_id: String,
        app_secret: String,
        base_url: String,
        cache: std::sync::Arc<MemoryTokenCache>,
    ) -> Self {
        Self {
            app_id,
            app_secret,
            base_url,
            cache,
            refresh_config: super::token::TokenRefreshConfig::default(),
        }
    }

    /// 设置刷新配置
    pub fn with_refresh_config(mut self, config: super::token::TokenRefreshConfig) -> Self {
        self.refresh_config = config;
        self
    }

    /// 刷新应用访问令牌
    pub async fn refresh_app_token(&self) -> AuthResult<TokenInfo> {
        self.refresh_token_internal(TokenType::AppAccessToken, None)
            .await
    }

    /// 刷新租户访问令牌
    pub async fn refresh_tenant_token(&self, tenant_key: &str) -> AuthResult<TokenInfo> {
        self.refresh_token_internal(TokenType::TenantAccessToken, Some(tenant_key))
            .await
    }

    /// 刷新用户访问令牌
    pub async fn refresh_user_token(&self, _refresh_token: &str) -> AuthResult<TokenInfo> {
        self.refresh_token_internal(TokenType::UserAccessToken, None)
            .await
    }

    /// 使用刷新令牌获取新的访问令牌
    pub async fn refresh_with_refresh_token(&self, refresh_token: &str) -> AuthResult<TokenInfo> {
        // 这是一个基础的实现框架，具体的HTTP调用需要在具体实现中完成
        debug!("Refreshing token with refresh_token");

        // 模拟HTTP响应
        let mock_response = RefreshTokenResponse {
            app_access_token: Some(format!("mock_app_token_{}", refresh_token)),
            tenant_access_token: None,
            refresh_token: Some(format!("new_refresh_{}", refresh_token)),
            expires_in: 3600,
            token_type: Some("app_access_token".to_string()),
        };

        self.parse_refresh_response(mock_response, TokenType::AppAccessToken)
    }

    /// 内部刷新方法
    async fn refresh_token_internal(
        &self,
        token_type: TokenType,
        tenant_key: Option<&str>,
    ) -> AuthResult<TokenInfo> {
        debug!("Refreshing token of type: {:?}", token_type);

        // 这里应该包含实际的HTTP调用逻辑
        // 目前提供一个模拟实现
        let mock_response = match (token_type, tenant_key) {
            (TokenType::AppAccessToken, None) => RefreshTokenResponse {
                app_access_token: Some(format!(
                    "mock_app_token_{}",
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                )),
                tenant_access_token: None,
                refresh_token: Some("mock_refresh_token".to_string()),
                expires_in: 3600,
                token_type: Some("app_access_token".to_string()),
            },
            (TokenType::TenantAccessToken, Some(tenant)) => RefreshTokenResponse {
                app_access_token: None,
                tenant_access_token: Some(format!(
                    "mock_tenant_token_{}_{}",
                    tenant,
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                )),
                refresh_token: Some("mock_refresh_token".to_string()),
                expires_in: 7200,
                token_type: Some("tenant_access_token".to_string()),
            },
            _ => {
                return Err(AuthError::TokenError(
                    "Unsupported token type or missing tenant key".to_string(),
                ));
            }
        };

        let token_info = self.parse_refresh_response(mock_response, token_type)?;

        // 缓存新令牌
        let cache_key = self.generate_cache_key(token_info.token_type, tenant_key);
        self.cache.put(&cache_key, token_info.clone()).await;

        info!(
            "Successfully refreshed {} token for tenant: {:?}",
            token_type.as_str(),
            tenant_key
        );

        Ok(token_info)
    }

    /// 解析刷新响应
    fn parse_refresh_response(
        &self,
        response: RefreshTokenResponse,
        expected_type: TokenType,
    ) -> AuthResult<TokenInfo> {
        let access_token = match expected_type {
            TokenType::AppAccessToken => response.app_access_token,
            TokenType::TenantAccessToken => response.tenant_access_token,
            TokenType::UserAccessToken => {
                return Err(AuthError::TokenError(
                    "User access token refresh not supported".to_string(),
                ));
            }
        };

        let access_token = access_token
            .ok_or_else(|| AuthError::TokenError("Missing access_token".to_string()))?;

        let expires_in = response.expires_in;
        let expires_at = SystemTime::now() + Duration::from_secs(expires_in);

        let mut token_info = TokenInfo {
            access_token,
            refresh_token: response.refresh_token,
            expires_at,
            token_type: expected_type,
            app_type: "openlark-auth".to_string(),
            tenant_key: None,
            created_at: SystemTime::now(),
            last_accessed_at: SystemTime::now(),
            access_count: 0,
        };

        // 设置租户密钥（如果是租户令牌）
        if expected_type == TokenType::TenantAccessToken {
            token_info.tenant_key = Some("default_tenant".to_string());
        }

        Ok(token_info)
    }

    /// 生成缓存键
    fn generate_cache_key(&self, token_type: TokenType, tenant_key: Option<&str>) -> String {
        match (token_type, tenant_key) {
            (TokenType::AppAccessToken, None) => "app_access_token".to_string(),
            (TokenType::TenantAccessToken, Some(tenant)) => {
                format!("tenant_access_token:{}", tenant)
            }
            (TokenType::UserAccessToken, None) => "user_access_token".to_string(),
            _ => "unknown_token".to_string(),
        }
    }

    /// 检查是否需要刷新
    pub async fn should_refresh(&self, token_info: &TokenInfo) -> bool {
        if token_info.is_expired() {
            return true;
        }

        if let Some(_refresh_token) = &token_info.refresh_token {
            // 检查是否在刷新提前期内
            let refresh_ahead = Duration::from_secs(self.refresh_config.refresh_ahead_seconds);
            let time_until_expiry = token_info.time_until_expiry();

            if let Some(remaining) = time_until_expiry {
                remaining < refresh_ahead
            } else {
                false
            }
        } else {
            false
        }
    }

    /// 重试刷新令牌（带指数退避）
    pub async fn refresh_with_retry(
        &self,
        token_type: TokenType,
        tenant_key: Option<&str>,
    ) -> AuthResult<TokenInfo> {
        let max_attempts = self.refresh_config.max_retry_attempts;
        let base_interval = Duration::from_secs(self.refresh_config.retry_interval_base);
        let max_interval = Duration::from_secs(self.refresh_config.retry_interval_max);

        for attempt in 1..=max_attempts {
            match self.refresh_token_internal(token_type, tenant_key).await {
                Ok(token) => {
                    if attempt > 1 {
                        info!("Token refresh succeeded on attempt {}", attempt);
                    }
                    return Ok(token);
                }
                Err(e) => {
                    if attempt == max_attempts {
                        error!(
                            "Token refresh failed after {} attempts: {}",
                            max_attempts, e
                        );
                        return Err(e);
                    } else {
                        warn!(
                            "Token refresh attempt {} failed, retrying...: {}",
                            attempt, e
                        );

                        // 指数退避等待
                        let delay =
                            std::cmp::min(base_interval * 2_u32.pow(attempt - 1), max_interval);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(AuthError::TokenError(
            "Max retry attempts reached".to_string(),
        ))
    }

    /// 批量刷新多个令牌（提高性能）
    pub async fn batch_refresh(
        &self,
        requests: Vec<(TokenType, Option<String>)>,
    ) -> Vec<AuthResult<TokenInfo>> {
        let mut results = Vec::with_capacity(requests.len());

        for (token_type, tenant_key) in requests {
            let result = self
                .refresh_token_internal(token_type, tenant_key.as_deref())
                .await;
            results.push(result);
        }

        results
    }

    /// 预热缓存（批量刷新常用令牌）
    pub async fn warmup_cache(&self, tenant_keys: Vec<String>) -> AuthResult<()> {
        // 批量刷新应用令牌
        let _ = self.refresh_app_token().await;

        // 批量刷新租户令牌
        let tenant_requests: Vec<_> = tenant_keys
            .into_iter()
            .map(|key| (TokenType::TenantAccessToken, Some(key)))
            .collect();

        let results = self.batch_refresh(tenant_requests).await;

        // 记录预热结果
        let successful_count = results.iter().filter(|r| r.is_ok()).count();
        info!(
            "Cache warmup completed: {}/{} tokens refreshed successfully",
            successful_count,
            results.len()
        );

        Ok(())
    }
}

/// TokenRefresherBuilder - 构建器模式创建TokenRefresher
pub struct TokenRefresherBuilder {
    app_id: Option<String>,
    app_secret: Option<String>,
    base_url: Option<String>,
    cache: Option<std::sync::Arc<MemoryTokenCache>>,
    refresh_config: Option<super::token::TokenRefreshConfig>,
}

impl TokenRefresherBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            app_id: None,
            app_secret: None,
            base_url: None,
            cache: None,
            refresh_config: None,
        }
    }

    /// 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        self.app_secret = Some(app_secret.into());
        self
    }

    /// 设置基础URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// 设置缓存
    pub fn cache(mut self, cache: std::sync::Arc<MemoryTokenCache>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// 设置刷新配置
    pub fn refresh_config(mut self, config: super::token::TokenRefreshConfig) -> Self {
        self.refresh_config = Some(config);
        self
    }

    /// 构建TokenRefresher
    pub fn build(self) -> AuthResult<TokenRefresher> {
        let app_id = self
            .app_id
            .ok_or_else(|| AuthError::ConfigError("app_id is required".to_string()))?;

        let app_secret = self
            .app_secret
            .ok_or_else(|| AuthError::ConfigError("app_secret is required".to_string()))?;

        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://open.feishu.cn".to_string());

        let cache = self.cache.unwrap_or_else(|| {
            std::sync::Arc::new(MemoryTokenCache::new(
                crate::auth::cache::CacheConfig::default(),
            ))
        });

        let mut refresher = TokenRefresher::new(app_id, app_secret, base_url, cache);

        if let Some(config) = self.refresh_config {
            refresher = refresher.with_refresh_config(config);
        }

        Ok(refresher)
    }
}

impl Default for TokenRefresherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_token_refresher_builder() {
        let cache = Arc::new(MemoryTokenCache::new(
            crate::auth::cache::CacheConfig::default(),
        ));

        let refresher = TokenRefresherBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://open.feishu.cn")
            .cache(cache)
            .build();

        assert!(refresher.is_ok());
    }

    #[tokio::test]
    async fn test_should_refresh_logic() {
        let cache = Arc::new(MemoryTokenCache::new(
            crate::auth::cache::CacheConfig::default(),
        ));
        let refresher = TokenRefresherBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .cache(cache)
            .build()
            .unwrap();

        // 测试已过期的令牌
        let expired_token = TokenInfo {
            access_token: "test".to_string(),
            refresh_token: Some("refresh".to_string()),
            expires_at: SystemTime::now() - Duration::from_secs(1),
            token_type: TokenType::AppAccessToken,
            app_type: "test".to_string(),
            tenant_key: None,
            created_at: SystemTime::now() - Duration::from_secs(3600),
            last_accessed_at: SystemTime::now(),
            access_count: 10,
        };

        assert!(refresher.should_refresh(&expired_token).await);

        // 测试有效的令牌
        let valid_token = TokenInfo {
            access_token: "test".to_string(),
            refresh_token: Some("refresh".to_string()),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            token_type: TokenType::AppAccessToken,
            app_type: "test".to_string(),
            tenant_key: None,
            created_at: SystemTime::now(),
            last_accessed_at: SystemTime::now(),
            access_count: 0,
        };

        assert!(!refresher.should_refresh(&valid_token).await);
    }

    #[tokio::test]
    async fn test_refresh_response_parsing() {
        let cache = Arc::new(MemoryTokenCache::new(
            crate::auth::cache::CacheConfig::default(),
        ));
        let refresher = TokenRefresherBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .cache(cache)
            .build()
            .unwrap();

        let response = RefreshTokenResponse {
            app_access_token: Some("test_token".to_string()),
            tenant_access_token: None,
            refresh_token: Some("refresh_token".to_string()),
            expires_in: 3600,
            token_type: Some("app_access_token".to_string()),
        };

        let result = refresher.parse_refresh_response(response, TokenType::AppAccessToken);
        assert!(result.is_ok());

        let token_info = result.unwrap();
        assert_eq!(token_info.access_token, "test_token");
        assert_eq!(token_info.token_type, TokenType::AppAccessToken);
        assert_eq!(token_info.refresh_token, Some("refresh_token".to_string()));
    }

    #[tokio::test]
    async fn test_cache_key_generation() {
        let cache = Arc::new(MemoryTokenCache::new(
            crate::auth::cache::CacheConfig::default(),
        ));
        let refresher = TokenRefresherBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .cache(cache)
            .build()
            .unwrap();

        // 测试应用令牌缓存键
        let app_key = refresher.generate_cache_key(TokenType::AppAccessToken, None);
        assert_eq!(app_key, "app_access_token");

        // 测试租户令牌缓存键
        let tenant_key =
            refresher.generate_cache_key(TokenType::TenantAccessToken, Some("tenant_123"));
        assert_eq!(tenant_key, "tenant_access_token:tenant_123");

        // 测试用户令牌缓存键
        let user_key = refresher.generate_cache_key(TokenType::UserAccessToken, None);
        assert_eq!(user_key, "user_access_token");
    }
}
