pub mod v1;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Bitable {
    service: Arc<DocsService>,
}

impl Bitable {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::BitableV1 {
        v1::BitableV1::new(self.service.clone())
    }
}