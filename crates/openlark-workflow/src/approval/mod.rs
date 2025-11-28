pub mod old;
pub mod v4;

use std::sync::Arc;
use crate::service::WorkflowService;

#[derive(Clone)]
pub struct Approval {
    service: Arc<WorkflowService>,
}

impl Approval {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    pub fn old(&self) -> old::ApprovalOLD {
        old::ApprovalOLD::new(self.service.clone())
    }

    pub fn v4(&self) -> v4::ApprovalV4 {
        v4::ApprovalV4::new(self.service.clone())
    }
}