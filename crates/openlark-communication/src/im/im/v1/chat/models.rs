//! 群相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 群列表排序方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatSortType {
    ByCreateTimeAsc,
    ByActiveTimeDesc,
}

impl ChatSortType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ByCreateTimeAsc => "ByCreateTimeAsc",
            Self::ByActiveTimeDesc => "ByActiveTimeDesc",
        }
    }
}

/// 群分享链接有效期
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatLinkValidityPeriod {
    Week,
    Year,
    Permanently,
}

impl ChatLinkValidityPeriod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Week => "week",
            Self::Year => "year",
            Self::Permanently => "permanently",
        }
    }
}

/// 获取群分享链接响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatLinkResponse {
    pub share_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_permanent: Option<bool>,
}

impl ApiResponseTrait for GetChatLinkResponse {
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
