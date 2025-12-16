//! # OpenLark 认证模块
//!
//! OpenLark SDK的认证和授权模块，提供飞书开放平台的完整认证解决方案。
//!
//! ## 功能特性
//!
//! - **令牌管理**: 自动处理访问令牌的获取、刷新和缓存
//! - **多种认证方式**: 支持企业自建应用、应用商店应用和用户认证
//! - **OAuth支持**: 完整的OAuth 2.0授权流程
//! - **类型安全**: 基于Rust类型系统的API设计
//! - **错误处理**: 统一的错误处理和用户友好的错误消息
//! - **异步支持**: 基于tokio的全异步API设计
//!
//! ## 模块组织
//!
//! - [`services`][]: 核心认证服务
//! - [`models`][]: 数据模型定义
//! - [`api`][]: API实现层
//!
//! ## 快速开始
//!
//! ```rust
//! use openlark_auth::{AuthService, AuthenService, OAuthService};
//! use openlark_core::Config;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::from_env()?;
//!
//!     // 企业自建应用认证
//!     let auth_service = AuthService::new(config.clone());
//!     let token = auth_service.v3()
//!         .app_access_token_internal()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .send()
//!         .await?;
//!
//!     // 用户认证
//!     let authen_service = AuthenService::new(config.clone());
//!     let user_info = authen_service.v1()
//!         .user_info()
//!         .get()
//!         .user_access_token("user_token")
//!         .send()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod common;
pub mod models;
pub mod services;
pub mod utils;

// 重新导出核心类型，方便用户使用
pub use services::{AuthService, AuthenService, OAuthService};

// 重新导出常用模型
pub use models::{auth::*, authen::*, oauth::*};

// 重新导出API构建器和服务
pub use api::*;

// 类型别名，提供更好的用户体验
pub type AuthResult<T> = openlark_core::SDKResult<T>;

/// 认证模块的预导入，包含最常用的类型和特征
pub mod prelude {
    pub use crate::{AuthResult, AuthService, AuthenService, OAuthService};

    // 重新导出API服务类
    pub use crate::api::{
        AppAccessTokenBuilder, AppAccessTokenInternalBuilder, AppTicketResendBuilder,
        AuthServiceV3, AuthenServiceV1, AuthorizationBuilder, OAuthServiceOld,
        OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder, OidcService,
        RefreshUserAccessTokenV1Builder, TenantAccessTokenBuilder,
        TenantAccessTokenInternalRequestBuilder, UserAccessTokenV1Builder, UserInfoBuilder,
    };

    // 重新导出openlark-core的核心类型
    pub use openlark_core::api::ApiResponse;
    pub use openlark_core::{
        config::Config,
        error::{CoreError as LarkAPIError, SDKResult},
        http::Transport,
    };

    // 重新导出常用模型
    pub use crate::models::{AccessTokenResponse, AuthorizationCodeRequest, UserInfo};
}

/// 认证模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use std::marker::PhantomData;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_prelude_imports() {
        // 确保prelude中的类型可以正常导入，避免unused import警告
        use crate::prelude::{AuthResult, AuthService, AuthenService, OAuthService};

        // 这里只是验证类型导入，不进行实际操作
        let _: String = VERSION.to_string();

        // 验证导入的类型存在
        let _ = PhantomData::<AuthService>;
        let _ = PhantomData::<AuthenService>;
        let _ = PhantomData::<OAuthService>;
        let _: PhantomData<AuthResult<()>>;
    }
}
