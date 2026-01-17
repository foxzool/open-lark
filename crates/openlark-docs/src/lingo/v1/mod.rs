pub mod classification;
pub mod draft;
pub mod entity;
pub mod file;
pub mod repo;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct LingoV1 {
    service: Arc<DocsService>,
}

impl LingoV1 {
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

    pub fn repo(&self) -> repo::Repo {
        repo::Repo::new(self.service.clone())
    }
}