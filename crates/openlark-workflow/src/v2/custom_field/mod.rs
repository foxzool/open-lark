pub mod add;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod option;
pub mod remove;
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// CustomField：自定义字段资源
#[derive(Clone)]
pub struct CustomField {
    config: Arc<Config>,
    tasklist_guid: String,
}

impl CustomField {
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

    pub fn create(&self) -> create::CreateCustomFieldRequest {
        create::CreateCustomFieldRequest::new(self.config.clone(), self.tasklist_guid.clone())
    }

    pub fn get(&self, field_guid: impl Into<String>) -> get::GetCustomFieldRequest {
        get::GetCustomFieldRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            field_guid.into(),
        )
    }

    pub fn update(&self, field_guid: impl Into<String>) -> update::UpdateCustomFieldRequest {
        update::UpdateCustomFieldRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            field_guid.into(),
        )
    }

    pub fn delete(&self, field_guid: impl Into<String>) -> delete::DeleteCustomFieldRequest {
        delete::DeleteCustomFieldRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            field_guid.into(),
        )
    }

    pub fn list(&self) -> list::ListCustomFieldsRequest {
        list::ListCustomFieldsRequest::new(self.config.clone(), self.tasklist_guid.clone())
    }

    /// 获取自定义字段选项资源（不需要 tasklist_guid）
    pub fn option(&self, custom_field_guid: impl Into<String>) -> option::CustomFieldOptionResource {
        option::CustomFieldOptionResource::new(self.config.clone(), custom_field_guid.into())
    }
}

// 重新导出请求类型
pub use add::AddCustomFieldRequest;
pub use create::CreateCustomFieldRequest;
pub use delete::DeleteCustomFieldRequest;
pub use get::GetCustomFieldRequest;
pub use list::ListCustomFieldsRequest;
pub use remove::RemoveCustomFieldRequest;
pub use update::UpdateCustomFieldRequest;

// 重新导出响应类型
pub use models::{
    CreateCustomFieldBody, CreateCustomFieldResponse, CustomFieldConfig, CustomFieldItem,
    CustomFieldType, DeleteCustomFieldResponse, GetCustomFieldResponse, ListCustomFieldsResponse,
    UpdateCustomFieldBody, UpdateCustomFieldResponse,
};
