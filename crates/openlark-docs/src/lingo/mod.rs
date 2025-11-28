pub mod v1;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Lingo {
    service: Arc<DocsService>,
}

impl Lingo {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::LingoV1 {
        v1::LingoV1::new(self.service.clone())
    }
}