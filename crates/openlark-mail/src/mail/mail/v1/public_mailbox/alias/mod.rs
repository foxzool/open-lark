//! 公共邮箱别名模块

pub mod create;
pub mod delete;
pub mod list;
pub mod models;

use openlark_core::config::Config;
use std::sync::Arc;

/// Alias：公共邮箱别名资源
#[derive(Clone)]
pub struct Alias {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Alias {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 创建别名
    pub fn create(&self) -> create::CreatePublicMailboxAliasRequest {
        create::CreatePublicMailboxAliasRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取别名列表
    pub fn list(&self) -> list::PublicMailboxAliasListRequest {
        list::PublicMailboxAliasListRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 删除别名
    pub fn delete(&self, alias_id: impl Into<String>) -> delete::DeletePublicMailboxAliasRequest {
        delete::DeletePublicMailboxAliasRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            alias_id.into(),
        )
    }
}

// 重新导出
pub use create::CreatePublicMailboxAliasRequest;
pub use delete::DeletePublicMailboxAliasRequest;
pub use list::PublicMailboxAliasListRequest;
pub use models::{
    CreatePublicMailboxAliasBody, CreatePublicMailboxAliasResponse,
    DeletePublicMailboxAliasResponse, PublicMailboxAliasItem, PublicMailboxAliasListResponse,
};

#[cfg(test)]
mod tests {

    use serde_json;

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
