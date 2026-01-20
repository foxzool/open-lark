pub mod create;
pub mod update;
pub mod delete;
pub mod get;
pub mod complete;
pub mod uncomplete;
pub mod models;

use std::sync::Arc;
use openlark_core::config::Config;

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
pub use create::CreateTaskRequest;
pub use update::UpdateTaskRequest;
pub use delete::DeleteTaskRequest;
pub use get::GetTaskRequest;
pub use complete::CompleteTaskRequest;
pub use uncomplete::UncompleteTaskRequest;

// 重新导出响应类型
pub use models::{
    CreateTaskBody, CreateTaskResponse,
    UpdateTaskBody, UpdateTaskResponse,
    DeleteTaskResponse, GetTaskResponse,
    CompleteTaskResponse, UncompleteTaskResponse,
};
