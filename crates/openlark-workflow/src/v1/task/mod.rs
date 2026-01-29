pub mod comment;
pub mod collaborator;
pub mod complete;
pub mod create;
pub mod delete;
pub mod follower;
pub mod get;
pub mod list;
pub mod patch;
pub mod reminder;
pub mod uncomplete;

pub use comment::*;
pub use collaborator::*;
pub use complete::*;
pub use create::*;
pub use delete::*;
pub use follower::*;
pub use get::*;
pub use list::*;
pub use patch::*;
pub use reminder::*;
pub use uncomplete::*;

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
