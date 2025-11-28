pub mod v1;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Docs {
    service: Arc<DocsService>,
}

impl Docs {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::DocsV1 {
        v1::DocsV1::new(self.service.clone())
    }
}
