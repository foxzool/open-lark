use std::sync::Arc;

use crate::core::config::Config;

pub use v1::V1;

pub mod v1;

/// 云文档助手服务
pub struct AssistantService {
    /// V1版本API
    pub v1: V1,
}

impl AssistantService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            v1: V1::new((*config).clone()),
        }
    }
}
