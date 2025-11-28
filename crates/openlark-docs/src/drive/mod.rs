pub mod v1;
pub mod v2;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Drive {
    service: Arc<DocsService>,
}

impl Drive {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::DriveV1 {
        v1::DriveV1::new(self.service.clone())
    }

    pub fn v2(&self) -> v2::DriveV2 {
        v2::DriveV2::new(self.service.clone())
    }
}
