pub mod collaborator;
pub mod comment;
pub mod complete;
pub mod create;
pub mod delete;
pub mod follower;
pub mod get;
pub mod list;
pub mod patch;
pub mod reminder;
pub mod uncomplete;

// 显式导出 Request 类型，避免 ambiguous glob re-exports
pub use collaborator::{
    BatchDeleteTaskCollaboratorRequestV1, CreateTaskCollaboratorRequestV1,
    DeleteTaskCollaboratorRequestV1, ListTaskCollaboratorRequestV1,
};
pub use comment::{
    CreateTaskCommentRequestV1, DeleteTaskCommentRequestV1, GetTaskCommentRequestV1,
    ListTaskCommentRequestV1, UpdateTaskCommentRequestV1,
};
pub use complete::CompleteTaskRequestV1;
pub use create::CreateTaskRequestV1;
pub use delete::DeleteTaskRequestV1;
pub use follower::{
    BatchDeleteTaskFollowerRequestV1, CreateTaskFollowerRequestV1, DeleteTaskFollowerRequestV1,
    ListTaskFollowerRequestV1,
};
pub use get::GetTaskRequestV1;
pub use list::ListTaskRequestV1;
pub use patch::UpdateTaskRequestV1;
pub use reminder::{
    CreateTaskReminderRequestV1, DeleteTaskReminderRequestV1, ListTaskReminderRequestV1,
};
pub use uncomplete::UncompleteTaskRequestV1;

use openlark_core::config::Config;
use std::sync::Arc;

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

    pub fn get(&self, task_id: impl Into<String>) -> GetTaskRequestV1 {
        GetTaskRequestV1::new(self.config.clone(), task_id)
    }

    pub fn update(&self, task_id: impl Into<String>) -> UpdateTaskRequestV1 {
        UpdateTaskRequestV1::new(self.config.clone(), task_id)
    }

    pub fn delete(&self, task_id: impl Into<String>) -> DeleteTaskRequestV1 {
        DeleteTaskRequestV1::new(self.config.clone(), task_id)
    }

    pub fn complete(&self, task_id: impl Into<String>) -> CompleteTaskRequestV1 {
        CompleteTaskRequestV1::new(self.config.clone(), task_id)
    }

    pub fn uncomplete(&self, task_id: impl Into<String>) -> UncompleteTaskRequestV1 {
        UncompleteTaskRequestV1::new(self.config.clone(), task_id)
    }

    pub fn list(&self) -> ListTaskRequestV1 {
        ListTaskRequestV1::new(self.config.clone())
    }

    /// 关注者相关方法
    pub fn follower_create(&self, task_id: impl Into<String>) -> CreateTaskFollowerRequestV1 {
        CreateTaskFollowerRequestV1::new(self.config.clone(), task_id)
    }

    pub fn follower_delete(
        &self,
        task_id: impl Into<String>,
        follower_id: impl Into<String>,
    ) -> DeleteTaskFollowerRequestV1 {
        DeleteTaskFollowerRequestV1::new(self.config.clone(), task_id, follower_id)
    }

    pub fn follower_list(&self, task_id: impl Into<String>) -> ListTaskFollowerRequestV1 {
        ListTaskFollowerRequestV1::new(self.config.clone(), task_id)
    }

    pub fn follower_batch_delete(
        &self,
        task_id: impl Into<String>,
    ) -> BatchDeleteTaskFollowerRequestV1 {
        BatchDeleteTaskFollowerRequestV1::new(self.config.clone(), task_id)
    }

    /// 协作者相关方法
    pub fn collaborator_create(
        &self,
        task_id: impl Into<String>,
    ) -> CreateTaskCollaboratorRequestV1 {
        CreateTaskCollaboratorRequestV1::new(self.config.clone(), task_id)
    }

    pub fn collaborator_delete(
        &self,
        task_id: impl Into<String>,
        collaborator_id: impl Into<String>,
    ) -> DeleteTaskCollaboratorRequestV1 {
        DeleteTaskCollaboratorRequestV1::new(self.config.clone(), task_id, collaborator_id)
    }

    pub fn collaborator_list(&self, task_id: impl Into<String>) -> ListTaskCollaboratorRequestV1 {
        ListTaskCollaboratorRequestV1::new(self.config.clone(), task_id)
    }

    pub fn collaborator_batch_delete(
        &self,
        task_id: impl Into<String>,
    ) -> BatchDeleteTaskCollaboratorRequestV1 {
        BatchDeleteTaskCollaboratorRequestV1::new(self.config.clone(), task_id)
    }

    /// 提醒相关方法
    pub fn reminder_create(&self, task_id: impl Into<String>) -> CreateTaskReminderRequestV1 {
        CreateTaskReminderRequestV1::new(self.config.clone(), task_id)
    }

    pub fn reminder_delete(
        &self,
        task_id: impl Into<String>,
        reminder_id: impl Into<String>,
    ) -> DeleteTaskReminderRequestV1 {
        DeleteTaskReminderRequestV1::new(self.config.clone(), task_id, reminder_id)
    }

    pub fn reminder_list(&self, task_id: impl Into<String>) -> ListTaskReminderRequestV1 {
        ListTaskReminderRequestV1::new(self.config.clone(), task_id)
    }

    /// 评论相关方法
    pub fn comment_create(&self, task_id: impl Into<String>) -> CreateTaskCommentRequestV1 {
        CreateTaskCommentRequestV1::new(self.config.clone(), task_id)
    }

    pub fn comment_get(
        &self,
        task_id: impl Into<String>,
        comment_id: impl Into<String>,
    ) -> GetTaskCommentRequestV1 {
        GetTaskCommentRequestV1::new(self.config.clone(), task_id, comment_id)
    }

    pub fn comment_update(
        &self,
        task_id: impl Into<String>,
        comment_id: impl Into<String>,
    ) -> UpdateTaskCommentRequestV1 {
        UpdateTaskCommentRequestV1::new(self.config.clone(), task_id, comment_id)
    }

    pub fn comment_delete(
        &self,
        task_id: impl Into<String>,
        comment_id: impl Into<String>,
    ) -> DeleteTaskCommentRequestV1 {
        DeleteTaskCommentRequestV1::new(self.config.clone(), task_id, comment_id)
    }

    pub fn comment_list(&self, task_id: impl Into<String>) -> ListTaskCommentRequestV1 {
        ListTaskCommentRequestV1::new(self.config.clone(), task_id)
    }
}

#[cfg(test)]
#[allow(unused_variables)]
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
    fn test_task_new() {
        let config = create_test_config();
        let _ = Task::new(config);
    }

    #[test]
    fn test_task_create() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.create();
    }

    #[test]
    fn test_task_get() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.get("task_123");
    }

    #[test]
    fn test_task_update() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.update("task_123");
    }

    #[test]
    fn test_task_delete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.delete("task_123");
    }

    #[test]
    fn test_task_complete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.complete("task_123");
    }

    #[test]
    fn test_task_uncomplete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.uncomplete("task_123");
    }

    #[test]
    fn test_task_list() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.list();
    }

    // Follower tests
    #[test]
    fn test_task_follower_create() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.follower_create("task_123");
    }

    #[test]
    fn test_task_follower_delete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.follower_delete("task_123", "follower_456");
    }

    #[test]
    fn test_task_follower_list() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.follower_list("task_123");
    }

    #[test]
    fn test_task_follower_batch_delete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.follower_batch_delete("task_123");
    }

    // Collaborator tests
    #[test]
    fn test_task_collaborator_create() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.collaborator_create("task_123");
    }

    #[test]
    fn test_task_collaborator_delete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.collaborator_delete("task_123", "collaborator_456");
    }

    #[test]
    fn test_task_collaborator_list() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.collaborator_list("task_123");
    }

    #[test]
    fn test_task_collaborator_batch_delete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.collaborator_batch_delete("task_123");
    }

    // Reminder tests
    #[test]
    fn test_task_reminder_create() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.reminder_create("task_123");
    }

    #[test]
    fn test_task_reminder_delete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.reminder_delete("task_123", "reminder_456");
    }

    #[test]
    fn test_task_reminder_list() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.reminder_list("task_123");
    }

    // Comment tests
    #[test]
    fn test_task_comment_create() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.comment_create("task_123");
    }

    #[test]
    fn test_task_comment_get() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.comment_get("task_123", "comment_456");
    }

    #[test]
    fn test_task_comment_update() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.comment_update("task_123", "comment_456");
    }

    #[test]
    fn test_task_comment_delete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.comment_delete("task_123", "comment_456");
    }

    #[test]
    fn test_task_comment_list() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.comment_list("task_123");
    }
}
