pub mod task;

use std::sync::Arc;
use openlark_core::config::Config;

/// TaskV1：任务 API v1 访问入口
#[derive(Clone)]
pub struct TaskV1 {
    config: Arc<Config>,
}

impl TaskV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问任务资源
    pub fn task(&self) -> task::Task {
        task::Task::new(self.config.clone())
    }
}
