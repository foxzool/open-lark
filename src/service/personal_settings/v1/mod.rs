pub mod system_status;

use crate::core::config::Config;

/// Personal Settings API v1版本服务
pub struct V1 {
    /// 系统状态服务
    pub system_status: system_status::SystemStatusService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            system_status: system_status::SystemStatusService::new(config),
        }
    }
}
