//! 工作城市相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 多语言内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I18nContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 工作城市信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_city_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 获取单个工作城市信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCityResponse {
    pub work_city: WorkCity,
}

impl ApiResponseTrait for WorkCityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取租户工作城市列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkCitiesResponse {
    #[serde(default)]
    pub items: Vec<WorkCity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListWorkCitiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

