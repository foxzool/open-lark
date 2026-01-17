pub mod v1;
pub mod v2;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Wiki {
    service: Arc<DocsService>,
}

impl Wiki {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::WikiV1 {
        v1::WikiV1::new(self.service.clone())
    }

    pub fn v2(&self) -> v2::WikiV2 {
        v2::WikiV2::new(self.service.clone())
    }
}