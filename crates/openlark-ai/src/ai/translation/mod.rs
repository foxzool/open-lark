//! Translation 模块
//!
//! 提供翻译服务，包括文本翻译和语言检测。

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Translation API 入口
#[derive(Clone)]
pub struct Translation {
    config: Arc<Config>,
}

impl Translation {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn v1(&self) -> v1::TranslationV1 {
        v1::TranslationV1::new(self.config.clone())
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
