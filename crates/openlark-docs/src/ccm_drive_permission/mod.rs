pub mod old;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct CcmDrivePermission {
    service: Arc<DocsService>,
}

impl CcmDrivePermission {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn old(&self) -> old::CcmDrivePermissionOLD {
        old::CcmDrivePermissionOLD::new(self.service.clone())
    }
}