//! OpenLark 认证服务模块
//!
//! 提供飞书开放平台的完整认证服务，包括企业应用认证、用户身份认证和OAuth授权。
//!
//! ## 架构设计
//!
//! 采用 Project-Version-Resource (PVR) 三层架构：
//!
//! ```text
//! openlark-auth/src/
//! ├── models/           # 共享数据模型
//! ├── auth/            # 企业应用认证 (Project)
//! │   └── v3/          # API版本v3 (Version)
//! ├── authen/          # 用户身份认证 (Project)
//! │   └── v1/          # API版本v1 (Version)
//! └── oauth/           # OAuth授权 (Project)
//!     └── old/         # 向后兼容版本 (Version)
//! ```
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use openlark_auth::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = AuthConfig::new("app_id", "app_secret");
//!     let auth = AuthServices::new(config);
//!
//!     // 获取自建应用租户访问令牌
//!     let tenant_token = auth.auth.v3().tenant_access_token()
//!         .internal()
//!         .send()
//!         .await?;
//!
//!     println!("租户令牌: {}", tenant_token.tenant_access_token);
//!     Ok(())
//! }
//! ```
//!
//! ## API覆盖
//!
//! ### auth (v3) - 企业应用认证
//! - `tenant_access_token_internal()` - 自建应用获取租户访问令牌
//! - `app_access_token_internal()` - 自建应用获取应用访问令牌
//! - `tenant_access_token()` - 商店应用获取租户访问令牌
//! - `app_access_token()` - 商店应用获取应用访问令牌
//! - `app_ticket_resend()` - 重新推送应用票据
//!
//! ### authen (v1) - 用户身份认证
//! - `user_info.get()` - 获取登录用户信息
//! - `oidc.create_access_token()` - 获取OIDC访问令牌
//! - `oidc.create_refresh_access_token()` - 刷新OIDC访问令牌
//! - `access_token.create()` - 获取用户访问令牌
//!
//! ### oauth (old) - OAuth授权
//! - `authorization.get_index()` - 获取登录预授权码

// #![deny(missing_docs)]  // 暂时禁用以完成基本编译
#![warn(clippy::all)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

// 错误处理模块
pub mod error;

// 共享数据模型
pub mod models;

// Project: auth - 企业应用认证
pub mod auth;

// Project: authen - 用户身份认证
pub mod authen;

// Project: oauth - OAuth授权
pub mod oauth;

// 重新导出主要类型
pub use auth::{AuthProject, AuthV3Service};
pub use authen::{AuthenProject, AuthenV1Service};
pub use oauth::{OauthOldService, OauthProject};

/// 认证服务统一入口
#[derive(Debug)]
pub struct AuthServices {
    pub config: std::sync::Arc<crate::models::AuthConfig>,
    pub auth: AuthProject,
    pub authen: AuthenProject,
    pub oauth: OauthProject,
}

impl AuthServices {
    /// 创建新的认证服务实例
    pub fn new(config: crate::models::AuthConfig) -> Self {
        let config = std::sync::Arc::new(config);

        Self {
            auth: AuthProject::new(config.clone()),
            authen: AuthenProject::new(config.clone()),
            oauth: OauthProject::new(config.clone()),
            config,
        }
    }

    /// 获取配置信息
    pub fn config(&self) -> &crate::models::AuthConfig {
        &self.config
    }
}

impl Default for AuthServices {
    fn default() -> Self {
        Self::new(crate::models::AuthConfig::default())
    }
}

// 结果类型别名现在通过 models::AuthResult 导出

/// 预导出模块
pub mod prelude {
    pub use super::models::AuthResult;
    pub use super::{AuthProject, AuthServices, AuthenProject, OauthProject};

    pub use super::auth::*;
    pub use super::authen::*;
    pub use super::models::*;
    pub use super::oauth::*;
}
