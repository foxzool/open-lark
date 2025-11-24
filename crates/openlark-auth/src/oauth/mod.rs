//! OAuth授权 (Project: oauth)
//!
//! 提供OAuth授权服务，向后兼容的第三方授权机制。

use std::sync::Arc;

/// OAuth授权项目入口
#[derive(Debug)]
pub struct OauthProject {
    pub config: Arc<crate::models::AuthConfig>,
    pub old: OauthOldService,
}

impl OauthProject {
    pub fn new(config: Arc<crate::models::AuthConfig>) -> Self {
        Self {
            old: OauthOldService::new(config.clone()),
            config,
        }
    }
}

/// old版本OAuth授权服务
#[derive(Debug)]
pub struct OauthOldService {
    config: Arc<crate::models::AuthConfig>,
}

impl OauthOldService {
    pub fn new(config: Arc<crate::models::AuthConfig>) -> Self {
        Self { config }
    }

    pub fn authorization(&self) -> crate::oauth::old::authorization::AuthorizationService {
        crate::oauth::old::authorization::AuthorizationService::new(self.config.clone())
    }
}

pub mod old;
