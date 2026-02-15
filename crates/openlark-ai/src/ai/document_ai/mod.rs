//! Document AI 模块

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Document AI API 入口
#[derive(Clone)]
pub struct DocumentAi {
    config: Arc<Config>,
}

impl DocumentAi {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn v1(&self) -> v1::DocumentAiV1 {
        v1::DocumentAiV1::new(self.config.clone())
    }
}
