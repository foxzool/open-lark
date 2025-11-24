//! 认证客户端

use std::sync::Arc;
use tracing::{debug, info, warn};

use super::config::AuthConfig;
use crate::auth::{
    cache::MemoryTokenCache,
    refresh::TokenRefresher,
    token::{AccessToken, TokenInfo, TokenType},
    validator::TokenValidator,
};
use crate::error::{AuthError, AuthResult};

/// 认证客户端
pub struct AuthClient {
    config: AuthConfig,
    cache: Arc<MemoryTokenCache>,
    refresher: TokenRefresher,
    validator: TokenValidator,
}

impl AuthClient {
    /// 创建新的认证客户端
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

    /// 验证令牌
    pub async fn validate_token(
        &self,
        token: &str,
    ) -> AuthResult<crate::auth::token::TokenValidationResult> {
        debug!("Validating token");

        // 首先验证令牌格式
        let format_result = self.validator.validate_token_format(token);
        if !format_result.is_valid() {
            return Ok(format_result);
        }

        // TODO: 这里可以添加更复杂的验证逻辑，比如调用飞书的验证接口
        // 目前返回基本验证结果
        Ok(format_result)
    }

    /// 刷新令牌
    pub async fn refresh_token(
        &self,
        token_type: TokenType,
        tenant_key: Option<&str>,
    ) -> AuthResult<TokenInfo> {
        debug!("Refreshing token of type: {:?}", token_type);

        match token_type {
            TokenType::AppAccessToken => {
                let token_info = self.refresher.refresh_app_token().await?;
                self.cache.put("app_access_token", token_info.clone()).await;
                Ok(token_info)
            }
            TokenType::TenantAccessToken => {
                let tenant_key = tenant_key.ok_or_else(|| {
                    AuthError::TokenError(
                        "tenant_key is required for tenant access token".to_string(),
                    )
                })?;
                let token_info = self.refresher.refresh_tenant_token(tenant_key).await?;
                let cache_key = format!("tenant_access_token:{}", tenant_key);
                self.cache.put(&cache_key, token_info.clone()).await;
                Ok(token_info)
            }
            TokenType::UserAccessToken => Err(AuthError::TokenError(
                "User access token refresh not supported".to_string(),
            )),
        }
    }

    /// 获取缓存统计信息
    pub async fn get_cache_stats(&self) -> crate::auth::cache::CacheStats {
        self.cache.stats().await
    }

    /// 清空缓存
    pub async fn clear_cache(&self) -> AuthResult<()> {
        debug!("Clearing all token cache");
        self.cache.clear().await;
        info!("Token cache cleared successfully");
        Ok(())
    }

    /// 预热缓存
    pub async fn warmup_cache(&self, tenant_keys: Vec<String>) -> AuthResult<()> {
        info!("Warming up cache for {} tenants", tenant_keys.len());
        self.refresher.warmup_cache(tenant_keys).await?;
        info!("Cache warmup completed");
        Ok(())
    }
}

/// 认证客户端构建器
pub struct AuthClientBuilder {
    config: Option<AuthConfig>,
}

impl AuthClientBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { config: None }
    }

    /// 设置配置
    pub fn config(mut self, config: AuthConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        let config = self.config.get_or_insert_with(AuthConfig::default);
        config.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        let config = self.config.get_or_insert_with(AuthConfig::default);
        config.app_secret = app_secret.into();
        self
    }

    /// 设置基础URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        let config = self.config.get_or_insert_with(AuthConfig::default);
        config.base_url = base_url.into();
        self
    }

    /// 构建认证客户端
    pub fn build(self) -> AuthResult<AuthClient> {
        let config = self
            .config
            .ok_or_else(|| AuthError::ConfigError("AuthConfig is required".to_string()))?;
        AuthClient::new(config)
    }
}

impl Default for AuthClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::client::config::AuthConfigBuilder;

    #[test]
    fn test_auth_client_builder() {
        let builder = AuthClientBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://open.feishu.cn");

        assert!(builder.build().is_ok());
    }

    #[test]
    fn test_auth_client_builder_missing_config() {
        let builder = AuthClientBuilder::new();
        assert!(builder.build().is_err());
    }

    #[tokio::test]
    async fn test_auth_client_creation() {
        let config = AuthConfigBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let client = AuthClient::new(config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let config = AuthConfigBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let client = AuthClient::new(config).unwrap();
        let stats = client.get_cache_stats().await;
        assert_eq!(stats.current_size, 0);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }
}
