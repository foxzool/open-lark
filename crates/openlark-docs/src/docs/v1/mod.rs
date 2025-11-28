pub mod content;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct DocsV1 {
    service: Arc<DocsService>,
}

impl DocsV1 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn content(&self) -> content::Content {
        content::Content::new(self.service.clone())
    }
}