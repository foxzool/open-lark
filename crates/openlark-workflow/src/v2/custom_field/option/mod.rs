//! 自定义字段选项模块

pub mod create;
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// CustomFieldOption：自定义字段选项资源
#[derive(Clone)]
pub struct CustomFieldOptionResource {
    config: Arc<Config>,
    custom_field_guid: String,
}

impl CustomFieldOptionResource {
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
        }
    }

    /// 创建自定义字段选项
    pub fn create(&self) -> create::CreateCustomFieldOptionRequest {
        create::CreateCustomFieldOptionRequest::new(
            self.config.clone(),
            self.custom_field_guid.clone(),
        )
    }

    /// 更新自定义字段选项
    pub fn patch(
        &self,
        option_guid: impl Into<String>,
    ) -> patch::UpdateCustomFieldOptionRequest {
        patch::UpdateCustomFieldOptionRequest::new(
            self.config.clone(),
            self.custom_field_guid.clone(),
            option_guid.into(),
        )
    }
}

// 重新导出请求类型
pub use create::CreateCustomFieldOptionRequest;
pub use patch::UpdateCustomFieldOptionRequest;

// 重新导出响应类型
pub use create::{
    CreateCustomFieldOptionBody, CreateCustomFieldOptionResponse, CustomFieldOption,
};
pub use patch::{UpdateCustomFieldOptionBody, UpdateCustomFieldOptionResponse};
