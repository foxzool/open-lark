pub mod info;

use openlark_core::core::config::Config;

/// Bot API v3版本服务
pub struct V3 {
    /// 机器人信息服务
    pub info: info::InfoService,


impl V3 {
    pub fn new(config: Config) -> Self {
        Self {
            info: info::InfoService::new(config),
        }
    }

