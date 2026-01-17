pub mod default;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct CardOLD {
    service: Arc<CommunicationService>,
}

impl CardOLD {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn default(&self) -> default::Default {
        default::Default::new(self.service.clone())
    }
}