//! 职级相关模型（不算 API）

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

/// 职级信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 创建/更新/获取单个职级响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLevelResponse {
    pub job_level: JobLevel,
}

impl ApiResponseTrait for JobLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取租户职级列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobLevelsResponse {
    #[serde(default)]
    pub items: Vec<JobLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListJobLevelsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

