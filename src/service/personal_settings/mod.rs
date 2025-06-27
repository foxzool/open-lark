pub mod models;
pub mod v1;

use crate::core::config::Config;

/// 个人设置服务
pub struct PersonalSettingsService {
    /// v1版本API
    pub v1: v1::V1,
}

impl PersonalSettingsService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
