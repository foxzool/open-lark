//! 群公告相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 获取群公告信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChatAnnouncementResponse {
    pub content: String,
    pub revision: String,
    pub create_time: String,
    pub update_time: String,
    pub owner_id_type: String,
    pub owner_id: String,
    pub modifier_id_type: String,
    pub modifier_id: String,
}

impl ApiResponseTrait for GetChatAnnouncementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新群公告信息请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchChatAnnouncementBody {
    pub revision: String,
    pub requests: Vec<String>,
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
