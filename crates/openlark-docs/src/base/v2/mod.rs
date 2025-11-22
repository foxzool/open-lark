//! Base V2 模块
//!
//! 提供Base应用的V2版本API，包括角色管理等功能。

use openlark_core::config::Config;

pub mod models;
pub mod role;

pub use models::*;
pub use role::*;

/// Base V2 服务
pub struct BaseV2Service {
    pub config: Config,
}

impl BaseV2Service {
    /// 创建Base V2服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取角色管理服务
    pub fn role_service(&self) -> role::RoleService {
        role::RoleService::new(self.config.clone())
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}