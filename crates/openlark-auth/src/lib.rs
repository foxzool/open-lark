//! OpenLark 认证模块
//!
//! 提供完整的飞书开放平台认证功能，包括：
//!
//! - **令牌管理**: 应用访问令牌、租户访问令牌、用户访问令牌
//! - **自动刷新**: 基于过期时间的自动令牌刷新
//! - **多级缓存**: 内存缓存和可选的 Redis 缓存
//! - **OAuth支持**: 完整的 OAuth 2.0 流程支持
//! - **安全验证**: 令牌有效性和安全性验证
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use openlark_auth::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> AuthResult<()> {
//!     // 从环境变量创建配置
//!     let config = AuthConfig::from_env()?;
//!
//!     // 创建认证客户端
//!     let auth_client = AuthClient::new(config)?;
//!
//!     // 获取应用访问令牌
//!     let token = auth_client
//!         .get_app_access_token()
//!         .await?;
//!
//!     println!("应用访问令牌: {}", token.app_access_token);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 功能特性
//!
//! ### 🔐 令牌管理
//! - **多种令牌类型**: 支持应用、租户、用户三种访问令牌
//! - **自动刷新**: 基于过期时间的智能刷新机制
//! - **线程安全**: 所有操作都是线程安全的
//! - **类型安全**: 强类型的令牌信息和请求/响应
//!
//! ### 🚀 高性能缓存
//! - **多级缓存**: 内存缓存 + Redis 缓存
//! - **智能过期**: 基于TTL的自动过期机制
//! - **批量操作**: 支持批量令牌操作
//! - **缓存统计**: 详细的缓存命中率和性能指标
//!
//! ### 🛡️ 安全验证
//! - **签名验证**: JWT 令牌签名验证
//! - **权限检查**: 令牌权限范围验证
//! - **时间验证**: 令牌有效期检查
//! - **加密存储**: 敏感数据加密缓存
//!
//! ### 🔌 OAuth 集成
//! - **标准流程**: 完整的 OAuth 2.0 授权流程
//! - **多平台支持**: Web 应用、移动应用、桌面应用
//! - **状态管理**: 安全的状态参数管理
//! - **回调处理**: 统一的回调处理接口
//!
//! ## 架构设计
//!
//! ```
//! openlark-auth/
//! ├── auth/           # 核心认证功能
//! │   ├── token.rs     # 令牌类型和管理
//!   ├── cache.rs     # 缓存实现
//!   ├── refresh.rs   # 刷新机制
//!   ├── validator.rs # 验证逻辑
//!   └── types.rs     # 类型定义
//! ├── client/         # 客户端接口
//! ├── endpoints/     # API端点定义
//! ├── managers/      # 业务管理器
//! └── utils/         # 工具函数
//! ```
//!
//! ## 使用示例
//!
//! ### 基础令牌管理
//!
//! ```rust,no_run
//! use openlark_auth::prelude::*;
//!
//! // 创建配置
//! let config = AuthConfig::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build()?;
//!
//! // 创建客户端
//! let client = AuthClient::new(config)?;
//!
//! // 获取应用访问令牌
//! let token = client.get_app_access_token().await?;
//! println!("令牌: {}", token.app_access_token);
//!
//! // 验证令牌
//! let validation = client.validate_token(&token.app_access_token).await?;
//! println!("验证结果: {}", validation.valid);
//! ```
//!
//! ### 高级缓存配置
//!
//! ```rust,no_run
//! use openlark_auth::prelude::*;
//!
//! // 配置多层缓存
//! let config = AuthConfig::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .memory_cache_config(MemoryCacheConfig {
//!         max_size: 1000,
//!         default_ttl: Duration::from_secs(3600),
//!     })
//!     .redis_cache_config(Some(RedisCacheConfig {
//!         url: "redis://localhost:6379",
//!         key_prefix: "openlark:",
//!         default_ttl: Duration::from_secs(7200),
//!     }))
//!     .build()?;
//!
//! let client = AuthClient::new(config)?;
//! ```
//!
//! ### OAuth 流程
//!
//! ```rust,no_run
//! use openlark_auth::prelude::*;
//!
//! let oauth = OAuthHandler::new("app_id", "app_secret");
//!
//! // 获取预授权码
//! let pre_auth = oauth.get_pre_auth_code(
//!     "https://your-domain.com/callback",
//!     "contact:base"
//! ).await?;
//!
//! // 构建授权URL
//! let auth_url = oauth.build_authorization_url(
//!     &pre_auth.pre_auth_code,
//!     "https://your-domain.com/callback",
//!     "contact:base"
//! );
//!
//! // 用户访问授权URL
//! println!("请访问: {}", auth_url);
//!
//! // 处理授权回调
//! let oauth_token = oauth.handle_callback(&auth_code).await?;
//! println!("OAuth令牌: {}", oauth_token.access_token);
//! ```

#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

// 核心模块
pub mod auth;
pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod managers;
pub mod services;
pub mod utils;

// 错误处理
pub use error::{AuthError, AuthResult};

// 核心类型
pub use auth::{
    cache::{CacheConfig, CacheStats, MemoryTokenCache, TokenCache, TokenStorage},
    refresh::{RefreshTokenResponse, TokenRefresher, TokenRefresherBuilder},
    token::{
        AccessToken, AppType, GetTokenRequest, RefreshToken, TokenInfo, TokenRefreshConfig, TokenType,
        TokenValidationResult,
    },
    types::{
        AuthContext, AuthValidationDetails, AuthValidationRequest, CacheStrategy, OAuthConfig,
        OAuthGrantType, OAuthRequest, OAuthResponse, PermissionScope, PreAuthCodeResponse,
        RefreshStrategy, TenantInfo, TokenExchangeRequest, TokenExchangeResponse,
        TokenSecurityConfig, TokenStorageLocation, UserInfo,
    },
    validator::TokenValidator,
};

// 客户端和管理器
pub use client::{AuthClient, AuthClientBuilder};
pub use managers::{CacheManager, RefreshManager, TokenManager};

// 配置
pub use config::{AuthConfig, AuthConfigBuilder};

// 端点
pub use endpoints::AuthEndpoints;

// 服务层
pub use services::AuthServices;

/// 🔧 预导出模块
///
/// 包含最常用的类型和特征，简化导入：
///
/// ```rust,no_run
/// use openlark_auth::prelude::*;
///
/// let config = AuthConfig::from_env()?;
/// let client = AuthClient::new(config)?;
/// ```
pub mod prelude {
    // 核心类型
    pub use crate::{
        AccessToken, AppType, AuthClient, AuthClientBuilder, AuthConfig, AuthResult, RefreshToken,
        TokenCache, TokenInfo, TokenManager, TokenRefresher, TokenType, TokenValidationResult,
        TokenValidator, AuthServices,
    };

    // 错误类型
    pub use crate::AuthError;

    // 配置构建器
    pub use crate::AuthConfigBuilder;

    // 特征定义
    pub use crate::{CacheManagement, TokenManagement};
}

/// 🔧 认证管理特征
///
/// 定义认证管理的核心接口
pub trait TokenManagement: Send + Sync {
    /// 获取访问令牌
    async fn get_access_token(&self, request: GetTokenRequest) -> AuthResult<AccessToken>;

    /// 刷新访问令牌
    async fn refresh_token(&self, refresh_token: &str) -> AuthResult<AccessToken>;

    /// 验证令牌
    async fn validate_token(&self, token: &str) -> AuthResult<TokenValidationResult>;

    /// 撤销令牌
    async fn revoke_token(&self, token: &str) -> AuthResult<()>;
}

/// 🔧 缓存管理特征
///
/// 定义令牌缓存的核心接口
pub trait CacheManagement: Send + Sync {
    /// 获取缓存的令牌
    async fn get_cached_token(&self, key: &str) -> AuthResult<Option<AccessToken>>;

    /// 缓存令牌
    async fn cache_token(
        &self,
        key: &str,
        token: &AccessToken,
        ttl: std::time::Duration,
    ) -> AuthResult<()>;

    /// 使缓存失效
    async fn invalidate_cache(&self, key: &str) -> AuthResult<()>;

    /// 清空所有缓存
    async fn clear_cache(&self) -> AuthResult<()>;

    /// 获取缓存统计信息
    async fn get_cache_stats(&self) -> AuthResult<CacheStats>;
}

/// 🏷️ 库信息
pub mod info {
    /// 库名称
    pub const NAME: &str = "OpenLark Auth";
    /// 库版本
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    /// 库描述
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    /// 仓库地址
    pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
}

/// 📦 版本兼容性信息
pub mod compatibility {
    /// 当前主要版本
    pub const MAJOR: u32 = 0;
    /// 当前次要版本
    pub const MINOR: u32 = 1;
    /// 当前补丁版本
    pub const PATCH: u32 = 0;
    /// 是否为开发版本
    pub const IS_DEV: bool = cfg!(debug_assertions);
    /// 版本字符串
    pub const VERSION_STRING: &str = "0.1.0-dev";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_info() {
        assert!(!info::NAME.is_empty());
        assert!(!info::VERSION.is_empty());
        assert!(!info::DESCRIPTION.is_empty());
    }

    #[test]
    fn test_prelude_reexports() {
        use prelude::*;

        // 基础类型应该可以导入
        let _client: AuthClientBuilder = AuthClientBuilder::new();
        let _config: AuthConfigBuilder = AuthConfigBuilder::new();

        // 创建默认配置
        let _config = AuthConfig::builder().app_id("test").build();
    }

    #[test]
    fn test_cache_stats_calculation() {
        let stats = CacheStats {
            hits: 80,
            misses: 20,
            cleanups: 0,
            current_size: 100,
        };

        assert_eq!(stats.hit_rate(), 0.8);
        // 测试向后兼容性
        assert_eq!(stats.hit_count(), 80);
        assert_eq!(stats.miss_count(), 20);
        assert_eq!(stats.total_items(), 100);
    }
}
