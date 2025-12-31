//! 序列相关模型（不算 API）

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

/// 序列信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFamily {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_family_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 创建/更新/获取单个序列响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFamilyResponse {
    pub job_family: JobFamily,
}

impl ApiResponseTrait for JobFamilyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取租户序列列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobFamiliesResponse {
    #[serde(default)]
    pub items: Vec<JobFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListJobFamiliesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

