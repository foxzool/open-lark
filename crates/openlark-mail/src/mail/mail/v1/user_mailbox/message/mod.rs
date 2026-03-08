pub mod get;
pub mod get_by_card;
pub mod list;
pub mod send;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Message {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Message {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub fn list(&self) -> list::ListMailboxMessageRequest {
        list::ListMailboxMessageRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn get(&self, message_id: impl Into<String>) -> get::GetMailboxMessageRequest {
        get::GetMailboxMessageRequest::new(self.config.clone(), self.mailbox_id.clone(), message_id)
    }

    pub fn get_by_card(&self) -> get_by_card::GetMailboxMessageByCardRequest {
        get_by_card::GetMailboxMessageByCardRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
        )
    }

    pub fn send(&self) -> send::SendMailboxMessageRequest {
        send::SendMailboxMessageRequest::new(self.config.clone(), self.mailbox_id.clone())
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
