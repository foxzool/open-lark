pub mod old;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ImEphemeral {
    service: Arc<CommunicationService>,
}

impl ImEphemeral {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn old(&self) -> old::ImEphemeralOLD {
        old::ImEphemeralOLD::new(self.service.clone())
    }
}
