use crate::core::config::Config;

pub mod models;
pub mod v1;

use v1::V1;

/// 视频会议服务
pub struct VcService {
    /// v1版本API
    pub v1: V1,
}

impl VcService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: V1::new(config),
        }
    }
}
