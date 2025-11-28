pub mod attachment;
pub mod employee;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct EhrV1 {
    service: Arc<HrService>,
}

impl EhrV1 {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn attachment(&self) -> attachment::Attachment {
        attachment::Attachment::new(self.service.clone())
    }

    pub fn employee(&self) -> employee::Employee {
        employee::Employee::new(self.service.clone())
    }
}