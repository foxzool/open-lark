pub mod v1;
pub mod v2;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct Corehr {
    service: Arc<HrService>,
}

impl Corehr {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::CorehrV1 {
        v1::CorehrV1::new(self.service.clone())
    }

    pub fn v2(&self) -> v2::CorehrV2 {
        v2::CorehrV2::new(self.service.clone())
    }
}