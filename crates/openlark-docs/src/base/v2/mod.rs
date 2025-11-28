pub mod app;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct BaseV2 {
    service: Arc<DocsService>,
}

impl BaseV2 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn app_role(&self) -> app::role::AppRole {
        app::role::AppRole::new(self.service.clone())
    }
}