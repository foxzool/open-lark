pub mod old;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct CcmDoc {
    service: Arc<DocsService>,
}

impl CcmDoc {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn old(&self) -> old::CcmDocOLD {
        old::CcmDocOLD::new(self.service.clone())
    }
}