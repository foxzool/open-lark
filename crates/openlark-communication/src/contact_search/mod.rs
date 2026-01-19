pub mod old;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ContactSearch {
    service: Arc<CommunicationService>,
}

impl ContactSearch {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn old(&self) -> old::ContactSearchOLD {
        old::ContactSearchOLD::new(self.service.clone())
    }
}
