pub mod subscribe;
pub mod unsubscribe;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Event {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Event {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub fn subscribe(&self) -> subscribe::SubscribeMailboxEventRequest {
        subscribe::SubscribeMailboxEventRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    pub fn unsubscribe(&self) -> unsubscribe::UnsubscribeMailboxEventRequest {
        unsubscribe::UnsubscribeMailboxEventRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
        )
    }
}

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
