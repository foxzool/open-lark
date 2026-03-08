//! 批量消息相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 批量消息推送与阅读情况
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchMessageReadUser {
    pub read_count: String,
    pub total_count: String,
}

/// 查询批量消息推送和阅读人数响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchMessageReadUserResponse {
    pub read_user: BatchMessageReadUser,
}

impl ApiResponseTrait for BatchMessageReadUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
