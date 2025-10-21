use crate::core::config::Config;

pub mod generate_caldav_conf;

/// 设置服务
pub struct SettingService {
    pub config: Config,
}

impl SettingService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
