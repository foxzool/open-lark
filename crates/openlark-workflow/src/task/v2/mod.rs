pub mod attachment;
pub mod comment;
pub mod custom_field;
pub mod section;
pub mod task;
pub mod tasklist;

use std::sync::Arc;
use crate::service::WorkflowService;

#[derive(Clone)]
pub struct TaskV2 {
    service: Arc<WorkflowService>,
}

impl TaskV2 {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    pub fn attachment(&self) -> attachment::Attachment {
        attachment::Attachment::new(self.service.clone())
    }

    pub fn comment(&self) -> comment::Comment {
        comment::Comment::new(self.service.clone())
    }

    pub fn custom_field(&self) -> custom_field::CustomField {
        custom_field::CustomField::new(self.service.clone())
    }

    pub fn custom_field_option(&self) -> custom_field::option::CustomFieldOption {
        custom_field::option::CustomFieldOption::new(self.service.clone())
    }

    pub fn section(&self) -> section::Section {
        section::Section::new(self.service.clone())
    }

    pub fn task(&self) -> task::Task {
        task::Task::new(self.service.clone())
    }

    pub fn task_subtask(&self) -> task::subtask::TaskSubtask {
        task::subtask::TaskSubtask::new(self.service.clone())
    }

    pub fn tasklist(&self) -> tasklist::Tasklist {
        tasklist::Tasklist::new(self.service.clone())
    }

    pub fn tasklist_activity_subscription(&self) -> tasklist::activity_subscription::TasklistActivitySubscription {
        tasklist::activity_subscription::TasklistActivitySubscription::new(self.service.clone())
    }
}