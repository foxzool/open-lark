pub mod models;
pub mod v1;

use crate::core::config::Config;

/// 邮箱服务
pub struct MailService {
    /// v1版本API
    pub v1: v1::V1,
}

impl MailService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
