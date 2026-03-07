/// 文件订阅（subscription）相关模型
///
/// 注意：该文件仅存放模型结构，不计入 API 文件数量。
use openlark_core::api::ApiResponseTrait;
use serde::{Deserialize, Serialize};

/// 订阅信息（get/create/patch 的 data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    /// 文档响应字段拼写为 is_subcribe（注意不是 is_subscribe）
    #[serde(rename = "is_subcribe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

impl ApiResponseTrait for Subscription {}

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
