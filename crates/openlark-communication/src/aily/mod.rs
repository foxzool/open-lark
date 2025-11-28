pub mod v1;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct Aily {
    service: Arc<CommunicationService>,
}

impl Aily {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::AilyV1 {
        v1::AilyV1::new(self.service.clone())
    }
}