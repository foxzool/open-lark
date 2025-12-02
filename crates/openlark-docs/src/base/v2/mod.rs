//! Base V2 模块
//!
//! 提供Base应用的V2版本API，包括角色管理等功能。

use openlark_core::config::Config;

pub mod app;
pub mod models;

pub use app::*;
pub use models::*;

/// Base V2 服务
pub struct BaseV2Service {
    pub config: Config,
}

impl BaseV2Service {
    /// 创建Base V2服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取应用管理服务
    pub fn app_service(&self) -> app::AppService {
        app::AppService::new(self.config.clone())
    }

    /// 获取角色管理服务（向后兼容）
    pub fn role_service(&self) -> app::role::RoleService {
        self.app_service().role_service()
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}
