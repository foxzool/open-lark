pub mod old;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct Card {
    service: Arc<CommunicationService>,
}

impl Card {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn old(&self) -> old::CardOLD {
        old::CardOLD::new(self.service.clone())
    }
}
