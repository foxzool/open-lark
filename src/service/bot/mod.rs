pub mod models;
pub mod v3;

use crate::core::config::Config;

/// 机器人服务
pub struct BotService {
    /// v3版本API
    pub v3: v3::V3,
}

impl BotService {
    pub fn new(config: Config) -> Self {
        Self {
            v3: v3::V3::new(config),
        }
    }
}
