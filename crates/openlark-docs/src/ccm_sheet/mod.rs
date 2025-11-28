pub mod old;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct CcmSheet {
    service: Arc<DocsService>,
}

impl CcmSheet {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn old(&self) -> old::CcmSheetOLD {
        old::CcmSheetOLD::new(self.service.clone())
    }
}
