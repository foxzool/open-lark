use crate::core::config::Config;

pub mod models;
pub mod v4;

use v4::V4;

/// 审批服务
pub struct ApprovalService {
    /// v4版本API
    pub v4: V4,
}

impl ApprovalService {
    pub fn new(config: Config) -> Self {
        Self {
            v4: V4::new(config),
        }
    }
}
