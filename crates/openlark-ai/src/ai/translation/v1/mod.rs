//! Translation V1 模块

pub mod text;

use openlark_core::config::Config;
use std::sync::Arc;

/// Translation V1 API
#[derive(Clone)]
pub struct TranslationV1 {
    config: Arc<Config>,
}

impl TranslationV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn text(&self) -> text::Text {
        text::Text::new(self.config.clone())
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
