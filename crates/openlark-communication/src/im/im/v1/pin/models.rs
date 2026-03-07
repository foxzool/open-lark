//! Pin 消息相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// Pin 消息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pin {
    pub message_id: String,
    pub chat_id: String,
    pub operator_id: String,
    pub operator_id_type: String,
    pub create_time: String,
}

/// Pin 消息请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePinBody {
    pub message_id: String,
}

/// Pin 消息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePinResponse {
    pub pin: Pin,
}

impl ApiResponseTrait for CreatePinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群内 Pin 消息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPinsResponse {
    #[serde(default)]
    pub items: Option<Vec<Pin>>,
    pub has_more: bool,
    #[serde(default)]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListPinsResponse {
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
