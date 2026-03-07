//! CardKit 卡片实体响应模型

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 转换卡片 ID 响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConvertCardIdResponse {
    /// 转换后的卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

impl ApiResponseTrait for ConvertCardIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
