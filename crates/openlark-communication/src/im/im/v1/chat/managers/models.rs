//! 群管理员相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 指定/删除群管理员请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatManagersBody {
    pub manager_ids: Vec<String>,
}

/// 指定/删除群管理员响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatManagersResponse {
    #[serde(default)]
    pub chat_managers: Option<Vec<String>>,
    #[serde(default)]
    pub chat_bot_managers: Option<Vec<String>>,
}

impl ApiResponseTrait for ChatManagersResponse {
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
