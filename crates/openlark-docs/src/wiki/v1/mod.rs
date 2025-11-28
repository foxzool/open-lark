pub mod node;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct WikiV1 {
    service: Arc<DocsService>,
}

impl WikiV1 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn node(&self) -> node::Node {
        node::Node::new(self.service.clone())
    }
}
