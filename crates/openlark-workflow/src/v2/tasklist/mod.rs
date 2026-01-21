pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Tasklist：任务清单资源
#[derive(Clone)]
pub struct Tasklist {
    config: Arc<Config>,
}

impl Tasklist {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn create(&self) -> create::CreateTasklistRequest {
        create::CreateTasklistRequest::new(self.config.clone())
    }

    pub fn get(&self, tasklist_guid: impl Into<String>) -> get::GetTasklistRequest {
        get::GetTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    pub fn update(&self, tasklist_guid: impl Into<String>) -> update::UpdateTasklistRequest {
        update::UpdateTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    pub fn delete(&self, tasklist_guid: impl Into<String>) -> delete::DeleteTasklistRequest {
        delete::DeleteTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    pub fn list(&self) -> list::ListTasklistsRequest {
        list::ListTasklistsRequest::new(self.config.clone())
    }
}

// 重新导出请求类型
pub use create::CreateTasklistRequest;
pub use delete::DeleteTasklistRequest;
pub use get::GetTasklistRequest;
pub use list::ListTasklistsRequest;
pub use update::UpdateTasklistRequest;

// 重新导出响应类型
pub use models::{
    CreateTasklistBody, CreateTasklistResponse, DeleteTasklistResponse, GetTasklistResponse,
    ListTasklistsResponse, TasklistIcon, TasklistItem, UpdateTasklistBody, UpdateTasklistResponse,
};
