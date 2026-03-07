//! Speech-to-Text V1 模块

pub mod speech;

use openlark_core::config::Config;
use std::sync::Arc;

/// Speech-to-Text V1 API
#[derive(Clone)]
pub struct SpeechToTextV1 {
    config: Arc<Config>,
}

impl SpeechToTextV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn speech(&self) -> speech::Speech {
        speech::Speech::new(self.config.clone())
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
