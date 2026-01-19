pub mod old;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ContactUser {
    service: Arc<CommunicationService>,
}

impl ContactUser {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn old(&self) -> old::ContactUserOLD {
        old::ContactUserOLD::new(self.service.clone())
    }
}
