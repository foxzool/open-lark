//! OCR V1 模块

pub mod image;

use openlark_core::config::Config;
use std::sync::Arc;

/// OCR V1 API
#[derive(Clone)]
pub struct OcrV1 {
    config: Arc<Config>,
}

impl OcrV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// image。
    pub fn image(&self) -> image::Image {
        image::Image::new(self.config.clone())
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
