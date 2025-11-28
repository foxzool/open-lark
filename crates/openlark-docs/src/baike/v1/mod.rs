pub mod classification;
pub mod draft;
pub mod entity;
pub mod file;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct BaikeV1 {
    service: Arc<DocsService>,
}

impl BaikeV1 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn classification(&self) -> classification::Classification {
        classification::Classification::new(self.service.clone())
    }

    pub fn draft(&self) -> draft::Draft {
        draft::Draft::new(self.service.clone())
    }

    pub fn entity(&self) -> entity::Entity {
        entity::Entity::new(self.service.clone())
    }

    pub fn file(&self) -> file::File {
        file::File::new(self.service.clone())
    }
}