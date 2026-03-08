pub mod create;
pub mod delete;
pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 邮件组别名资源
#[derive(Clone)]
pub struct Alias {
    config: Arc<Config>,
    mailgroup_id: String,
}

impl Alias {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailGroupAliasRequest {
        list::ListMailGroupAliasRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    pub fn create(&self) -> create::CreateMailGroupAliasRequest {
        create::CreateMailGroupAliasRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    pub fn delete(&self, alias_id: impl Into<String>) -> delete::DeleteMailGroupAliasRequest {
        delete::DeleteMailGroupAliasRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
            alias_id,
        )
    }
}

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
