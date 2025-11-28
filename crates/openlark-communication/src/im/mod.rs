pub mod v1;
pub mod v2;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct Im {
    service: Arc<CommunicationService>,
}

impl Im {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::ImV1 {
        v1::ImV1::new(self.service.clone())
    }

    pub fn v2(&self) -> v2::ImV2 {
        v2::ImV2::new(self.service.clone())
    }
}