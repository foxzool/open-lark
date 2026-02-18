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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        )
    }

    #[test]
    fn test_section_new() {
        let config = create_test_config();
        let section = Section::new(config);
        assert!(section.tasklist_guid.is_empty());
    }

    #[test]
    fn test_section_with_tasklist() {
        let config = create_test_config();
        let section = Section::new(config).with_tasklist("tasklist_123");
        assert_eq!(section.tasklist_guid, "tasklist_123");
    }

    #[test]
    fn test_section_create() {
        let config = create_test_config();
        let section = Section::new(config).with_tasklist("tasklist_123");
        let _request = section.create();
    }

    #[test]
    fn test_section_get() {
        let config = create_test_config();
        let section = Section::new(config).with_tasklist("tasklist_123");
        let _request = section.get("section_456");
    }

    #[test]
    fn test_section_update() {
        let config = create_test_config();
        let section = Section::new(config).with_tasklist("tasklist_123");
        let _request = section.update("section_456");
    }

    #[test]
    fn test_section_delete() {
        let config = create_test_config();
        let section = Section::new(config).with_tasklist("tasklist_123");
        let _request = section.delete("section_456");
    }

    #[test]
    fn test_section_list() {
        let config = create_test_config();
        let section = Section::new(config).with_tasklist("tasklist_123");
        let _request = section.list();
    }
}
