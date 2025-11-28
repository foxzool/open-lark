pub mod v1;

use std::sync::Arc;
use crate::service::HrService;

#[derive(Clone)]
pub struct Ehr {
    service: Arc<HrService>,
}

impl Ehr {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::EhrV1 {
        v1::EhrV1::new(self.service.clone())
    }
}