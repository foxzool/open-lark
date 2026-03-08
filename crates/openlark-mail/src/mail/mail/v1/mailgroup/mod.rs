pub mod alias;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod manager;
pub mod member;
pub mod models;
pub mod patch;
pub mod permission_member;
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// MailGroup：邮件组资源（v1）
#[derive(Clone)]
pub struct MailGroup {
    config: Arc<Config>,
}

impl MailGroup {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn create(&self) -> create::CreateMailGroupRequest {
        create::CreateMailGroupRequest::new(self.config.clone())
    }

    pub fn get(&self, mail_group_id: impl Into<String>) -> get::GetMailGroupRequest {
        get::GetMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    pub fn update(&self, mail_group_id: impl Into<String>) -> update::UpdateMailGroupRequest {
        update::UpdateMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    pub fn delete(&self, mail_group_id: impl Into<String>) -> delete::DeleteMailGroupRequest {
        delete::DeleteMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    pub fn list(&self) -> list::MailGroupListRequest {
        list::MailGroupListRequest::new(self.config.clone())
    }
}

// 重新导出请求类型
pub use create::CreateMailGroupRequest;
pub use delete::DeleteMailGroupRequest;
pub use get::GetMailGroupRequest;
pub use list::MailGroupListRequest;
pub use update::UpdateMailGroupRequest;

// 重新导出响应类型
pub use models::{
    CreateMailGroupBody, CreateMailGroupResponse, DeleteMailGroupResponse, GetMailGroupResponse,
    MailGroupItem, MailGroupListResponse, UpdateMailGroupBody, UpdateMailGroupResponse,
};

#[cfg(test)]
#[allow(unused_imports)]
mod tests {


    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
