//! Authen v1 API实现

pub mod oidc;
pub mod user_info;
pub mod access_token;
pub mod refresh_access_token;

// 重新导出子模块的构建器和服务
pub use user_info::{UserInfoBuilder, UserInfoService};
pub use access_token::UserAccessTokenV1Builder;
pub use refresh_access_token::RefreshUserAccessTokenV1Builder;
pub use oidc::{OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder, OidcService};

use openlark_core::config::Config;

// 类型别名
pub type AuthResult<T> = openlark_core::error::SDKResult<T>;

// AuthenServiceV1结构体
#[derive(Debug)]
pub struct AuthenServiceV1 {
    config: Config,
}

impl AuthenServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 用户信息服务
    pub fn user_info(&self) -> UserInfoService {
        UserInfoService::new(self.config.clone())
    }

    /// 用户访问令牌构建器（v1版本）
    pub fn access_token(&self) -> UserAccessTokenV1Builder {
        UserAccessTokenV1Builder::new(self.config.clone())
    }

    /// 用户访问令牌刷新构建器（v1版本）
    pub fn refresh_access_token(&self) -> RefreshUserAccessTokenV1Builder {
        RefreshUserAccessTokenV1Builder::new(self.config.clone())
    }

    /// OIDC服务
    pub fn oidc(&self) -> OidcService {
        OidcService::new(self.config.clone())
    }
}