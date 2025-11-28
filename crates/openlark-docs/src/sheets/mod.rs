pub mod v3;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct Sheets {
    service: Arc<DocsService>,
}

impl Sheets {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn v3(&self) -> v3::SheetsV3 {
        v3::SheetsV3::new(self.service.clone())
    }
}