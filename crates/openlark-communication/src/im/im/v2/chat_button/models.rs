//! 消息流卡片按钮相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 单个用户失败原因
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FailedUserReason {
    /// 错误码。
    pub error_code: i32,
    /// 错误信息。
    pub error_message: String,
    /// 失败用户 ID。
    pub user_id: String,
}

/// 更新按钮响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatButtonUpdateResponse {
    /// 失败用户原因列表。
    #[serde(default)]
    pub failed_user_reasons: Option<Vec<FailedUserReason>>,
}

impl ApiResponseTrait for ChatButtonUpdateResponse {
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
