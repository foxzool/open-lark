pub mod complete;
pub mod create;
pub mod delete;
pub mod get;
pub mod models;
pub mod uncomplete;
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Task：任务资源（v2）
#[derive(Clone)]
pub struct Task {
    config: Arc<Config>,
}

impl Task {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn create(&self) -> create::CreateTaskRequest {
        create::CreateTaskRequest::new(self.config.clone())
    }

    pub fn update(&self, task_guid: impl Into<String>) -> update::UpdateTaskRequest {
        update::UpdateTaskRequest::new(self.config.clone(), task_guid.into())
    }

    pub fn delete(&self, task_guid: impl Into<String>) -> delete::DeleteTaskRequest {
        delete::DeleteTaskRequest::new(self.config.clone(), task_guid.into())
    }

    pub fn get(&self, task_guid: impl Into<String>) -> get::GetTaskRequest {
        get::GetTaskRequest::new(self.config.clone(), task_guid.into())
    }

    pub fn complete(&self, task_guid: impl Into<String>) -> complete::CompleteTaskRequest {
        complete::CompleteTaskRequest::new(self.config.clone(), task_guid.into())
    }

    pub fn uncomplete(&self, task_guid: impl Into<String>) -> uncomplete::UncompleteTaskRequest {
        uncomplete::UncompleteTaskRequest::new(self.config.clone(), task_guid.into())
    }
}

// 重新导出请求类型
pub use complete::CompleteTaskRequest;
pub use create::CreateTaskRequest;
pub use delete::DeleteTaskRequest;
pub use get::GetTaskRequest;
pub use uncomplete::UncompleteTaskRequest;
pub use update::UpdateTaskRequest;

// 重新导出响应类型
pub use models::{
    CompleteTaskResponse, CreateTaskBody, CreateTaskResponse, DeleteTaskResponse, GetTaskResponse,
    UncompleteTaskResponse, UpdateTaskBody, UpdateTaskResponse,
};
