#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Authorizations - 权限管理服务
//!
//! 提供权限管理相关的所有功能，包括：
//! - 用户授权管理
//! - 角色管理
//! - 权限范围管理

use crate::config::Config;

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
