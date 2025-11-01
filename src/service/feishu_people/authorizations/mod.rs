//! Authorizations - 权限管理服务
//!
//! 提供权限管理相关的所有功能，包括：
//! - 用户授权管理
//! - 角色管理
//! - 权限范围管理

use crate::core::config::Config;

/// 权限管理服务
#[derive(Debug, Clone)]
pub struct AuthorizationsService {
    config: Config,
}

impl AuthorizationsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// v1版本API
pub mod v1;
