pub mod default;

use std::sync::Arc;
use crate::service::WorkflowService;

#[derive(Clone)]
pub struct ApprovalOLD {
    service: Arc<WorkflowService>,
}

impl ApprovalOLD {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    pub fn default(&self) -> default::Default {
        default::Default::new(self.service.clone())
    }
}