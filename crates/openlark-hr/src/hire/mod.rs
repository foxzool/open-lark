pub mod v1;
pub mod v2;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct Hire {
    service: Arc<HrService>,
}

impl Hire {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::HireV1 {
        v1::HireV1::new(self.service.clone())
    }

    pub fn v2(&self) -> v2::HireV2 {
        v2::HireV2::new(self.service.clone())
    }
}