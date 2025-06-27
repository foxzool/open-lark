pub mod models;
pub mod v1;

use crate::core::config::Config;

/// 服务台服务
pub struct HelpdeskService {
    /// v1版本API
    pub v1: v1::V1,
}

impl HelpdeskService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
