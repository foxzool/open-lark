pub mod v1;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Docx {
    service: Arc<DocsService>,
}

impl Docx {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v1(&self) -> v1::DocxV1 {
        v1::DocxV1::new(self.service.clone())
    }
}