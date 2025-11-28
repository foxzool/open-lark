pub mod default;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ImMessageOLD {
    service: Arc<CommunicationService>,
}

impl ImMessageOLD {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn default(&self) -> default::Default {
        default::Default::new(self.service.clone())
    }
}