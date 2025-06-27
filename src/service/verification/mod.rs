pub mod models;
pub mod v1;

use crate::core::config::Config;

/// 认证信息服务
pub struct VerificationService {
    /// v1版本API
    pub v1: v1::V1,
}

impl VerificationService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
