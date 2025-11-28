pub mod v1;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct Payroll {
    service: Arc<HrService>,
}

impl Payroll {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::PayrollV1 {
        v1::PayrollV1::new(self.service.clone())
    }
}