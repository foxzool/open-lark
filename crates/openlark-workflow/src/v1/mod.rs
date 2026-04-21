//! 任务 v1 入口模块
//!
//! 提供任务 API v1 的统一访问入口，封装 v1 版本任务资源实例与调用起点。
//!
//! ## 主要功能
//! - `task`: 任务资源模块入口
//! - `TaskV1`: 任务 v1 服务访问封装

/// 任务模块。
pub mod task;

use openlark_core::config::Config;
use std::sync::Arc;

/// TaskV1：任务 API v1 访问入口
#[derive(Clone)]
pub struct TaskV1 {
    config: Arc<Config>,
}

impl TaskV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问任务资源
    pub fn task(&self) -> task::Task {
        task::Task::new(self.config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
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
