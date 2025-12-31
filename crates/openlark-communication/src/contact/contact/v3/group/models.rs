//! 用户组相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 用户组信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_user_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_department_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 查询指定用户组响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupResponse {
    pub group: Group,
}

impl ApiResponseTrait for GetGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建用户组响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupResponse {
    pub group_id: String,
}

impl ApiResponseTrait for CreateGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询用户组列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleListGroupsResponse {
    #[serde(default)]
    pub grouplist: Vec<Group>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for SimpleListGroupsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询用户所属用户组响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberBelongGroupsResponse {
    #[serde(default)]
    pub group_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl ApiResponseTrait for MemberBelongGroupsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
