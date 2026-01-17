pub mod old;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ImMessage {
    service: Arc<CommunicationService>,
}

impl ImMessage {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn old(&self) -> old::ImMessageOLD {
        old::ImMessageOLD::new(self.service.clone())
    }
}