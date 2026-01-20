pub mod create;

pub use create::*;

use std::sync::Arc;
use openlark_core::config::Config;

/// Task：任务资源（v1）
#[derive(Clone)]
pub struct Task {
    config: Arc<Config>,
}

impl Task {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn create(&self) -> CreateTaskRequestV1 {
        CreateTaskRequestV1::new(self.config.clone())
    }
}
