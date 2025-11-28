pub mod v1;
pub mod v2;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct Performance {
    service: Arc<HrService>,
}

impl Performance {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::PerformanceV1 {
        v1::PerformanceV1::new(self.service.clone())
    }

    pub fn v2(&self) -> v2::PerformanceV2 {
        v2::PerformanceV2::new(self.service.clone())
    }
}