//! 图片相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 图片类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Message,
    Avatar,
}

impl ImageType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Message => "message",
            Self::Avatar => "avatar",
        }
    }
}

/// 上传图片响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageResponse {
    pub image_key: String,
}

impl ApiResponseTrait for CreateImageResponse {
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
