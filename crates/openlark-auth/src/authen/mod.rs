//! 用户身份认证 (Project: authen)
//!
//! 提供用户身份认证服务，包括用户信息获取和OIDC认证。

use std::sync::Arc;

/// 用户身份认证项目入口
#[derive(Debug)]
pub struct AuthenProject {
    pub config: Arc<crate::models::AuthConfig>,
    pub v1: AuthenV1Service,
}

impl AuthenProject {
    pub fn new(config: Arc<crate::models::AuthConfig>) -> Self {
        Self {
            v1: AuthenV1Service::new(config.clone()),
            config,
        }
    }
}

/// v1版本用户身份认证服务
#[derive(Debug)]
pub struct AuthenV1Service {
    config: Arc<crate::models::AuthConfig>,
}

impl AuthenV1Service {
    pub fn new(config: Arc<crate::models::AuthConfig>) -> Self {
        Self { config }
    }

    pub fn user_info(&self) -> crate::authen::v1::user_info::UserInfoService {
        crate::authen::v1::user_info::UserInfoService::new(self.config.clone())
    }

    pub fn oidc(&self) -> crate::authen::v1::oidc::OidcService {
        crate::authen::v1::oidc::OidcService::new(self.config.clone())
    }

    pub fn access_token(&self) -> crate::authen::v1::access_token::AccessTokenService {
        crate::authen::v1::access_token::AccessTokenService::new(self.config.clone())
    }
}

pub mod v1;
