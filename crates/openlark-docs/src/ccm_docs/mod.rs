pub mod old;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct CcmDocs {
    service: Arc<DocsService>,
}

impl CcmDocs {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn old(&self) -> old::CcmDocsOLD {
        old::CcmDocsOLD::new(self.service.clone())
    }
}
