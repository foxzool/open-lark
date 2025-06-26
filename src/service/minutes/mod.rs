use crate::core::config::Config;

pub mod models;
pub mod v1;

use v1::V1;

/// 妙记服务
pub struct MinutesService {
    /// v1版本API
    pub v1: V1,
}

impl MinutesService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: V1::new(config),
        }
    }
}
