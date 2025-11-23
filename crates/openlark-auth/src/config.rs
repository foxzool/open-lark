//! 配置管理模块
//!
//! 提供认证模块的配置管理功能。

use crate::error::{AuthError, AuthResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 认证配置
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 基础URL
    pub base_url: String,
    /// 请求超时时间
    pub timeout: Duration,
    /// 内存缓存配置
    pub memory_cache_config: Option<MemoryCacheConfig>,
    /// Redis缓存配置
    pub redis_cache_config: Option<RedisCacheConfig>,
    /// 功能标志
    pub feature_flags: std::collections::HashMap<String, bool>,
}

/// 内存缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryCacheConfig {
    /// 最大缓存项数
    pub max_size: usize,
    /// 默认TTL
    pub default_ttl: Duration,
    /// 清理间隔
    pub cleanup_interval: Duration,
}

impl Default for MemoryCacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            default_ttl: Duration::from_secs(3600),
            cleanup_interval: Duration::from_secs(300),
        }
    }
}

/// Redis缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisCacheConfig {
    /// Redis连接URL
    pub url: String,
    /// 键前缀
    pub key_prefix: String,
    /// 默认TTL
    pub default_ttl: Duration,
    /// 连接池大小
    pub pool_size: Option<usize>,
}

/// 认证配置构建器
pub struct AuthConfigBuilder {
    app_id: Option<String>,
    app_secret: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    memory_cache_config: Option<MemoryCacheConfig>,
    redis_cache_config: Option<RedisCacheConfig>,
    feature_flags: std::collections::HashMap<String, bool>,
}

impl AuthConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self {
            app_id: None,
            app_secret: None,
            base_url: None,
            timeout: None,
            memory_cache_config: None,
            redis_cache_config: None,
            feature_flags: std::collections::HashMap::new(),
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

    /// 设置超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// 设置内存缓存配置
    pub fn memory_cache_config(mut self, config: MemoryCacheConfig) -> Self {
        self.memory_cache_config = Some(config);
        self
    }

    /// 设置Redis缓存配置
    pub fn redis_cache_config(mut self, config: RedisCacheConfig) -> Self {
        self.redis_cache_config = Some(config);
        self
    }

    /// 设置功能标志
    pub fn feature_flag<K: Into<String>>(mut self, key: K, value: bool) -> Self {
        self.feature_flags.insert(key.into(), value);
        self
    }

    /// 从环境变量构建配置
    pub fn from_env(mut self) -> AuthResult<Self> {
        if let Ok(app_id) = std::env::var("APP_ID") {
            self.app_id = Some(app_id);
        }
        if let Ok(app_secret) = std::env::var("APP_SECRET") {
            self.app_secret = Some(app_secret);
        }
        if let Ok(base_url) = std::env::var("LARK_BASE_URL") {
            self.base_url = Some(base_url);
        }
        Ok(self)
    }

    /// 构建最终配置
    pub fn build(self) -> AuthResult<AuthConfig> {
        let app_id = self
            .app_id
            .ok_or_else(|| AuthError::ConfigError("app_id is required".to_string()))?;

        let app_secret = self
            .app_secret
            .ok_or_else(|| AuthError::ConfigError("app_secret is required".to_string()))?;

        Ok(AuthConfig {
            app_id,
            app_secret,
            base_url: self
                .base_url
                .unwrap_or_else(|| "https://open.feishu.cn".to_string()),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            memory_cache_config: self.memory_cache_config,
            redis_cache_config: self.redis_cache_config,
            feature_flags: self.feature_flags,
        })
    }
}

impl Default for AuthConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthConfig {
    /// 创建配置构建器
    pub fn builder() -> AuthConfigBuilder {
        AuthConfigBuilder::new()
    }

    /// 从环境变量创建配置
    pub fn from_env() -> AuthResult<Self> {
        Self::builder().from_env()?.build()
    }
}
