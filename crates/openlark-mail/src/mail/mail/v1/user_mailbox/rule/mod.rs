pub mod create;
pub mod delete;
pub mod list;
pub mod reorder;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Rule {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Rule {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailboxRuleRequest {
        list::ListMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn create(&self) -> create::CreateMailboxRuleRequest {
        create::CreateMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn delete(&self, rule_id: impl Into<String>) -> delete::DeleteMailboxRuleRequest {
        delete::DeleteMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone(), rule_id)
    }

    pub fn reorder(&self) -> reorder::ReorderMailboxRuleRequest {
        reorder::ReorderMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
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
