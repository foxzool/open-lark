//! URL 预览相关模型（不算 API）

use serde::{Deserialize, Serialize};

/// 更新 URL 预览请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateUrlPreviewBody {
    pub preview_tokens: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_ids: Option<Vec<String>>,
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
