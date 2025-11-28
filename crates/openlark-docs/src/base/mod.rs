pub mod v2;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Base {
    service: Arc<DocsService>,
}

impl Base {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v2(&self) -> v2::BaseV2 {
        v2::BaseV2::new(self.service.clone())
    }
}