pub mod role;

use openlark_core::config::Config;

#[derive(Debug, Clone)]
pub struct AppService {
    config: Config,
}

impl AppService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 角色管理
    pub fn role(&self) -> crate::base::base::v2::app::role::RoleService {
        crate::base::base::v2::app::role::RoleService::new(self.config.clone())
    }
}
