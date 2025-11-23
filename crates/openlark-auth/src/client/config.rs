//! 认证配置模块

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 认证配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 基础URL
    pub base_url: String,
    /// 是否启用HTTPS
    pub enable_https: bool,
    /// 连接超时时间
    pub connect_timeout: Duration,
    /// 请求超时时间
    pub request_timeout: Duration,
    /// 最大重试次数
    pub max_retries: u32,
    /// 是否启用缓存
    pub enable_cache: bool,
    /// 缓存TTL
    pub cache_ttl: Duration,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            base_url: "https://open.feishu.cn".to_string(),
            enable_https: true,
            connect_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            max_retries: 3,
            enable_cache: true,
            cache_ttl: Duration::from_secs(3600),
        }
    }
}

impl AuthConfig {
    /// 从环境变量创建配置
    pub fn from_env() -> crate::error::AuthResult<Self> {
        let app_id = std::env::var("APP_ID").map_err(|_| {
            crate::error::AuthError::ConfigError("APP_ID environment variable not set".to_string())
        })?;
        let app_secret = std::env::var("APP_SECRET").map_err(|_| {
            crate::error::AuthError::ConfigError(
                "APP_SECRET environment variable not set".to_string(),
            )
        })?;

        Ok(Self {
            app_id,
            app_secret,
            ..Default::default()
        })
    }
}

/// 认证配置构建器
pub struct AuthConfigBuilder {
    config: AuthConfig,
}

impl AuthConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self {
            config: AuthConfig::default(),
        }
    }

    /// 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.config.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        self.config.app_secret = app_secret.into();
        self
    }

    /// 设置基础URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.config.base_url = base_url.into();
        self
    }

    /// 设置连接超时
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.config.connect_timeout = timeout;
        self
    }

    /// 设置请求超时
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.config.request_timeout = timeout;
        self
    }

    /// 设置最大重试次数
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    /// 构建配置
    pub fn build(self) -> crate::error::AuthResult<AuthConfig> {
        if self.config.app_id.is_empty() {
            return Err(crate::error::AuthError::ConfigError(
                "app_id is required".to_string(),
            ));
        }
        if self.config.app_secret.is_empty() {
            return Err(crate::error::AuthError::ConfigError(
                "app_secret is required".to_string(),
            ));
        }
        Ok(self.config)
    }
}

impl Default for AuthConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_config_builder() {
        let config = AuthConfigBuilder::new()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://open.feishu.cn")
            .build()
            .unwrap();

        assert_eq!(config.app_id, "test_app_id");
        assert_eq!(config.app_secret, "test_app_secret");
        assert_eq!(config.base_url, "https://open.feishu.cn");
    }

    #[test]
    fn test_auth_config_default() {
        let config = AuthConfig::default();
        assert_eq!(config.base_url, "https://open.feishu.cn");
        assert!(config.enable_https);
        assert_eq!(config.max_retries, 3);
        assert!(config.enable_cache);
    }

    #[test]
    fn test_build_config_without_app_id() {
        let result = AuthConfigBuilder::new().app_secret("test_secret").build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::error::AuthError::ConfigError(_)
        ));
    }
}
