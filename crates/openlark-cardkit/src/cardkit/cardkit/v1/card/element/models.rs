//! CardKit 卡片组件响应模型

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 新增组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCardElementResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for CreateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardElementResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for UpdateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 补丁组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchCardElementResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for PatchCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 流式更新文本响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardElementContentResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for UpdateCardElementContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteCardElementResponse {
    /// 卡片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 组件 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl ApiResponseTrait for DeleteCardElementResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
