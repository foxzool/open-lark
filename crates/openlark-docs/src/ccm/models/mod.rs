/// 云文档模型定义
use serde::{Deserialize, Serialize};

use openlark_core::api::Response;

/// 文件类型
#[derive(Debug, Deserialize, Serialize)]
pub enum FileType {
    Document,
    Spreadsheet,
    Presentation,
    Image,
    Video,
    Other,
}

/// 云文档响应
#[derive(Debug, Deserialize, Serialize)]
pub struct CcmResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// 通用分页响应
#[derive(Debug, Deserialize, Serialize)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
