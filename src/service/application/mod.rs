pub mod models;
pub mod v6;

use crate::core::config::Config;

/// 应用信息服务
pub struct ApplicationService {
    /// v6版本API
    pub v6: v6::V6,
}

impl ApplicationService {
    pub fn new(config: Config) -> Self {
        Self {
            v6: v6::V6::new(config),
        }
    }
}
