pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Section：分组资源
#[derive(Clone)]
pub struct Section {
    config: Arc<Config>,
    tasklist_guid: String,
}

impl Section {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            tasklist_guid: String::new(),
        }
    }

    pub fn with_tasklist(mut self, tasklist_guid: impl Into<String>) -> Self {
        self.tasklist_guid = tasklist_guid.into();
        self
    }

    pub fn create(&self) -> create::CreateSectionRequest {
        create::CreateSectionRequest::new(self.config.clone(), self.tasklist_guid.clone())
    }

    pub fn get(&self, section_guid: impl Into<String>) -> get::GetSectionRequest {
        get::GetSectionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            section_guid.into(),
        )
    }

    pub fn update(&self, section_guid: impl Into<String>) -> update::UpdateSectionRequest {
        update::UpdateSectionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            section_guid.into(),
        )
    }

    pub fn delete(&self, section_guid: impl Into<String>) -> delete::DeleteSectionRequest {
        delete::DeleteSectionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            section_guid.into(),
        )
    }

    pub fn list(&self) -> list::ListSectionsRequest {
        list::ListSectionsRequest::new(self.config.clone(), self.tasklist_guid.clone())
    }
}

// 重新导出请求类型
pub use create::CreateSectionRequest;
pub use delete::DeleteSectionRequest;
pub use get::GetSectionRequest;
pub use list::ListSectionsRequest;
pub use patch::UpdateSectionRequest;

// 重新导出响应类型
pub use models::{
    CreateSectionBody, CreateSectionResponse, DeleteSectionResponse, GetSectionResponse,
    ListSectionsResponse, SectionItem, UpdateSectionBody, UpdateSectionResponse,
};
