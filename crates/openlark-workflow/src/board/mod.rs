pub mod v1;

use std::sync::Arc;
use crate::service::WorkflowService;

#[derive(Clone)]
pub struct Board {
    service: Arc<WorkflowService>,
}

impl Board {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::BoardV1 {
        v1::BoardV1::new(self.service.clone())
    }
}