pub mod approval;
pub mod external_approval;
pub mod external_instance;
pub mod external_task;
pub mod instance;
pub mod task;

use std::sync::Arc;
use crate::service::WorkflowService;

#[derive(Clone)]
pub struct ApprovalV4 {
    service: Arc<WorkflowService>,
}

impl ApprovalV4 {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    pub fn approval(&self) -> approval::Approval {
        approval::Approval::new(self.service.clone())
    }

    pub fn external_approval(&self) -> external_approval::ExternalApproval {
        external_approval::ExternalApproval::new(self.service.clone())
    }

    pub fn external_instance(&self) -> external_instance::ExternalInstance {
        external_instance::ExternalInstance::new(self.service.clone())
    }

    pub fn external_task(&self) -> external_task::ExternalTask {
        external_task::ExternalTask::new(self.service.clone())
    }

    pub fn instance(&self) -> instance::Instance {
        instance::Instance::new(self.service.clone())
    }

    pub fn instance_comment(&self) -> instance::comment::InstanceComment {
        instance::comment::InstanceComment::new(self.service.clone())
    }

    pub fn task(&self) -> task::Task {
        task::Task::new(self.service.clone())
    }
}