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
        get::GetCommentRequest::new(
            self.config.clone(),
            self.task_guid.clone(),
            comment_guid.into(),
        )
    }

    pub fn update(&self, comment_guid: impl Into<String>) -> update::UpdateCommentRequest {
        update::UpdateCommentRequest::new(
            self.config.clone(),
            self.task_guid.clone(),
            comment_guid.into(),
        )
    }

    pub fn delete(&self, comment_guid: impl Into<String>) -> delete::DeleteCommentRequest {
        delete::DeleteCommentRequest::new(
            self.config.clone(),
            self.task_guid.clone(),
            comment_guid.into(),
        )
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
    fn test_comment_new() {
        let config = create_test_config();
        let comment = Comment::new(config);
        assert!(comment.task_guid.is_empty());
    }

    #[test]
    fn test_comment_with_task() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        assert_eq!(comment.task_guid, "task_123");
    }

    #[test]
    fn test_comment_create() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.create();
    }

    #[test]
    fn test_comment_get() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.get("comment_456");
    }

    #[test]
    fn test_comment_update() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.update("comment_456");
    }

    #[test]
    fn test_comment_delete() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.delete("comment_456");
    }

    #[test]
    fn test_comment_list() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.list();
    }
}
