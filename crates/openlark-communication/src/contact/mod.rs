pub mod old;
pub mod v3;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct Contact {
    service: Arc<CommunicationService>,
}

impl Contact {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn old(&self) -> old::ContactOLD {
        old::ContactOLD::new(self.service.clone())
    }

    pub fn v3(&self) -> v3::ContactV3 {
        v3::ContactV3::new(self.service.clone())
    }
}