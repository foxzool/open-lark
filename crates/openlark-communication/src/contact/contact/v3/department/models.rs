//! 部门相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 部门名称国际化配置（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentI18nName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 部门信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<DepartmentI18nName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_ids: Option<Vec<String>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 部门详情响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentResponse {
    pub department: Department,
}

impl ApiResponseTrait for DepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 部门列表响应 data（分页）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentListResponse {
    #[serde(default)]
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<Department>,
}

impl ApiResponseTrait for DepartmentListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取部门信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetDepartmentsResponse {
    #[serde(default)]
    pub items: Vec<Department>,
}

impl ApiResponseTrait for BatchGetDepartmentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
