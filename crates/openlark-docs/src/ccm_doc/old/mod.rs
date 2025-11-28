pub mod default;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct CcmDocOLD {
    service: Arc<DocsService>,
}

impl CcmDocOLD {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn default(&self) -> default::Default {
        default::Default::new(self.service.clone())
    }
}