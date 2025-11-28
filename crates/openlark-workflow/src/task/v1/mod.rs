pub mod task;

use std::sync::Arc;
use crate::service::WorkflowService;

#[derive(Clone)]
pub struct TaskV1 {
    service: Arc<WorkflowService>,
}

impl TaskV1 {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    pub fn task(&self) -> task::Task {
        task::Task::new(self.service.clone())
    }

    pub fn task_collaborator(&self) -> task::collaborator::TaskCollaborator {
        task::collaborator::TaskCollaborator::new(self.service.clone())
    }

    pub fn task_comment(&self) -> task::comment::TaskComment {
        task::comment::TaskComment::new(self.service.clone())
    }

    pub fn task_follower(&self) -> task::follower::TaskFollower {
        task::follower::TaskFollower::new(self.service.clone())
    }

    pub fn task_reminder(&self) -> task::reminder::TaskReminder {
        task::reminder::TaskReminder::new(self.service.clone())
    }
}