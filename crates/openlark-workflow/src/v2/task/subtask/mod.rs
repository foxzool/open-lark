//! 子任务模块

pub mod create;
pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// Subtask：子任务资源
#[derive(Clone)]
pub struct Subtask {
    config: Arc<Config>,
    task_guid: String,
}

impl Subtask {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
        }
    }

    /// 创建子任务
    pub fn create(&self,
    ) -> create::CreateSubtaskRequest {
        create::CreateSubtaskRequest::new(self.config.clone(), self.task_guid.clone())
    }

    /// 获取子任务列表
    pub fn list(&self) -> list::ListSubtasksRequest {
        list::ListSubtasksRequest::new(self.config.clone(), self.task_guid.clone())
    }
}

// 重新导出请求类型
pub use create::CreateSubtaskRequest;
pub use list::ListSubtasksRequest;

// 重新导出响应类型
pub use create::CreateSubtaskResponse;
pub use list::ListSubtasksResponse;
