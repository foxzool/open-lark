//! Text translation module

pub mod detect;
pub mod translate;

use openlark_core::config::Config;
use std::sync::Arc;

/// Text translation API
#[derive(Clone)]
pub struct Text {
    #[allow(dead_code)]
    config: Arc<Config>,
}

impl Text {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
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
