pub mod v1;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Baike {
    service: Arc<DocsService>,
}

impl Baike {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::BaikeV1 {
        v1::BaikeV1::new(self.service.clone())
    }
}