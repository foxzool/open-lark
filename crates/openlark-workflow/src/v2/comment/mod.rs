pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Comment：评论资源
#[derive(Clone)]
pub struct Comment {
    config: Arc<Config>,
    task_guid: String,
}

impl Comment {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            task_guid: String::new(),
        }
    }

    pub fn with_task(mut self, task_guid: impl Into<String>) -> Self {
        self.task_guid = task_guid.into();
        self
    }

    pub fn create(&self) -> create::CreateCommentRequest {
        create::CreateCommentRequest::new(self.config.clone(), self.task_guid.clone())
    }

    pub fn get(&self, comment_guid: impl Into<String>) -> get::GetCommentRequest {
        get::GetCommentRequest::new(self.config.clone(), self.task_guid.clone(), comment_guid.into())
    }

    pub fn update(&self, comment_guid: impl Into<String>) -> update::UpdateCommentRequest {
        update::UpdateCommentRequest::new(self.config.clone(), self.task_guid.clone(), comment_guid.into())
    }

    pub fn delete(&self, comment_guid: impl Into<String>) -> delete::DeleteCommentRequest {
        delete::DeleteCommentRequest::new(self.config.clone(), self.task_guid.clone(), comment_guid.into())
    }

    pub fn list(&self) -> list::ListCommentsRequest {
        list::ListCommentsRequest::new(self.config.clone(), self.task_guid.clone())
    }
}

// 重新导出请求类型
pub use create::CreateCommentRequest;
pub use delete::DeleteCommentRequest;
pub use get::GetCommentRequest;
pub use list::ListCommentsRequest;
pub use update::UpdateCommentRequest;

// 重新导出响应类型
pub use models::{
    CommentItem, CreateCommentBody, CreateCommentResponse, DeleteCommentResponse,
    GetCommentResponse, ListCommentsResponse, UpdateCommentBody, UpdateCommentResponse,
};
