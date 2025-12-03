//! 企业应用认证 (Project: auth)
//!
//! 提供企业应用的认证功能，包括应用访问令牌和租户访问令牌。

use crate::models::AuthConfig;
use std::sync::Arc;

// v3 API版本模块
pub mod v3;

/// 企业应用认证项目
#[derive(Debug)]
pub struct AuthProject {
    config: Arc<AuthConfig>,
}

impl AuthProject {
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self { config }
    }

    /// 访问 v3 API
    pub fn v3(&self) -> AuthV3Service {
        AuthV3Service {
            config: self.config.clone(),
        }
    }
}

/// v3 API 服务
#[derive(Debug)]
pub struct AuthV3Service {
    config: Arc<AuthConfig>,
}

impl AuthV3Service {
    /// 租户访问令牌服务
    pub fn tenant_access_token(
        &self,
    ) -> crate::auth::v3::auth::TenantAccessTokenService {
        crate::auth::v3::auth::TenantAccessTokenService::from_auth_config((*self.config).clone())
    }

    /// 应用访问令牌服务
    pub fn app_access_token(&self) -> crate::auth::v3::auth::AppAccessTokenService {
        crate::auth::v3::auth::AppAccessTokenService::from_auth_config((*self.config).clone())
    }

    /// 应用票据服务
    pub fn app_ticket(&self) -> crate::auth::v3::auth::AppTicketService {
        crate::auth::v3::auth::AppTicketService::from_auth_config((*self.config).clone())
    }
}
