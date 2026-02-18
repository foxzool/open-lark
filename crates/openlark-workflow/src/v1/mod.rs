pub mod task;

use openlark_core::config::Config;
use std::sync::Arc;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        )
    }

    #[test]
    fn test_task_v1_new() {
        let config = create_test_config();
        let _task_v1 = TaskV1::new(config);
    }

    #[test]
    fn test_task_v1_task() {
        let config = create_test_config();
        let task_v1 = TaskV1::new(config);
        let _task = task_v1.task();
    }
}
