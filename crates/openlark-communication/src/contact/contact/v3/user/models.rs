//! 用户相关模型（不算 API）

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// 用户信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<UserAvatar>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 用户头像
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAvatar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 用户 ID 类型（查询参数）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    OpenId,
    UnionId,
    UserId,
}

impl UserIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenId => "open_id",
            Self::UnionId => "union_id",
            Self::UserId => "user_id",
        }
    }
}

/// 部门 ID 类型（查询参数）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepartmentIdType {
    DepartmentId,
    OpenDepartmentId,
}

impl DepartmentIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DepartmentId => "department_id",
            Self::OpenDepartmentId => "open_department_id",
        }
    }
}
