pub mod v1;
pub mod v2;

use std::sync::Arc;
use crate::service::WorkflowService;

#[derive(Clone)]
pub struct Task {
    service: Arc<WorkflowService>,
}

impl Task {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::TaskV1 {
        v1::TaskV1::new(self.service.clone())
    }

    pub fn v2(&self) -> v2::TaskV2 {
        v2::TaskV2::new(self.service.clone())
    }
}