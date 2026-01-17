pub mod v1;

use crate::service::HrService;
use std::sync::Arc;

#[derive(Clone)]
pub struct Compensation {
    service: Arc<HrService>,
}

impl Compensation {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    pub fn v1(&self) -> v1::CompensationV1 {
        v1::CompensationV1::new(self.service.clone())
    }
}
