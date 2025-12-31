//! 人员类型相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 国际化内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I18nContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 人员类型枚举值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeTypeEnum {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_content: Option<Vec<I18nContent>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 创建/更新人员类型响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeTypeEnumResponse {
    pub employee_type_enum: EmployeeTypeEnum,
}

impl ApiResponseTrait for EmployeeTypeEnumResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询人员类型响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEmployeeTypeEnumsResponse {
    #[serde(default)]
    pub items: Vec<EmployeeTypeEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListEmployeeTypeEnumsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
