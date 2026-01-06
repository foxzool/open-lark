//! 单位相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 单位信息
///
/// 字段随文档演进，未显式建模字段使用 `extra` 透传。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub unit_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 创建单位响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUnitResponse {
    pub unit_id: String,
}

impl ApiResponseTrait for CreateUnitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单位信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUnitResponse {
    pub unit: Unit,
}

impl ApiResponseTrait for GetUnitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单位列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUnitsResponse {
    #[serde(default)]
    pub unitlist: Vec<Unit>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListUnitsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 单位绑定的部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDepartment {
    pub unit_id: String,
    pub department_id: String,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 获取单位绑定的部门列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUnitDepartmentsResponse {
    #[serde(default)]
    pub departmentlist: Vec<UnitDepartment>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListUnitDepartmentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
