//! 职务相关模型（不算 API）

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

/// 职务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobTitle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 获取单个职务信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobTitleResponse {
    pub job_title: JobTitle,
}

impl ApiResponseTrait for JobTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取租户职务列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobTitlesResponse {
    #[serde(default)]
    pub items: Vec<JobTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListJobTitlesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

