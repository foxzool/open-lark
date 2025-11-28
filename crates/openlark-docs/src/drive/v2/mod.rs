pub mod file;
pub mod permission;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct DriveV2 {
    service: Arc<DocsService>,
}

impl DriveV2 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn file_like(&self) -> file::like::FileLike {
        file::like::FileLike::new(self.service.clone())
    }

    pub fn permission_public(&self) -> permission::public::PermissionPublic {
        permission::public::PermissionPublic::new(self.service.clone())
    }
}
