/// Base V2 应用模块
///
/// 提供基础应用（base）V2版本的应用管理功能，包括角色管理等。
use openlark_core::config::Config;

pub mod role;

pub use role::*;

/// 应用管理服务
pub struct AppService {
    pub config: Config,
}

impl AppService {
    /// 创建应用管理服务
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
