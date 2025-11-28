pub mod old;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct CcmDriveExplorer {
    service: Arc<DocsService>,
}

impl CcmDriveExplorer {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn old(&self) -> old::CcmDriveExplorerOLD {
        old::CcmDriveExplorerOLD::new(self.service.clone())
    }
}