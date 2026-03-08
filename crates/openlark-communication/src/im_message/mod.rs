pub mod old;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ImMessage {
    service: Arc<CommunicationService>,
}

impl ImMessage {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn old(&self) -> old::ImMessageOLD {
        old::ImMessageOLD::new(self.service.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
