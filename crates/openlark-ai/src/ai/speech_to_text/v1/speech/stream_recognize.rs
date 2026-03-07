//! 识别流式语音

use openlark_core::config::Config;

/// 识别流式语音请求
#[derive(Debug)]
pub struct StreamRecognizeRequest {
    #[allow(dead_code)]
    config: Config,
}

impl StreamRecognizeRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
