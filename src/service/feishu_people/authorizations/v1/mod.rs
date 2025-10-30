//! Authorizations v1 - 权限管理v1版本API
//!
//! 提供权限管理相关的所有功能，包括：
//! - 用户授权管理
//! - 角色管理
//! - 权限范围管理

use crate::core::config::Config;

/// 权限管理v1版本服务
#[derive(Debug, Clone)]
pub struct AuthorizationsV1Service {
    config: Config,
}

impl AuthorizationsV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}