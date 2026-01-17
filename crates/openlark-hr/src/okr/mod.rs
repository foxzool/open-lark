pub mod v1;

use crate::service::HrService;
use std::sync::Arc;

#[derive(Clone)]
pub struct Okr {
    service: Arc<HrService>,
}

impl Okr {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    pub fn v1(&self) -> v1::OkrV1 {
        v1::OkrV1::new(self.service.clone())
    }
}
