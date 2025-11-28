pub mod v1;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Minutes {
    service: Arc<DocsService>,
}

impl Minutes {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::MinutesV1 {
        v1::MinutesV1::new(self.service.clone())
    }
}