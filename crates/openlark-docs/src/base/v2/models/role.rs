//! Base V2 角色管理数据模型
//!
//! 定义了 Base V2 版本角色管理相关的数据结构，
//! 包括角色信息、请求和响应模型。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Role {
    /// 角色 ID
    pub role_id: String,
    /// 角色名称
    pub name: String,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色权限列表
    pub permissions: Vec<String>,
    /// 角色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 是否为内置角色
    pub is_builtin: bool,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

/// 创建角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleRequest {
    /// 角色名称
    pub name: String,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色权限列表
    pub permissions: Vec<String>,
    /// 角色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 扩展属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

impl CreateRoleRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("角色名称不能为空".to_string());
        }

        if self.name.len() > 100 {
            return Err("角色名称长度不能超过100个字符".to_string());
        }

        if let Some(ref desc) = self.description {
            if desc.len() > 500 {
                return Err("角色描述长度不能超过500个字符".to_string());
            }
        }

        if self.permissions.is_empty() {
            return Err("角色权限列表不能为空".to_string());
        }

        Ok(())
    }
}

/// 更新角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoleRequest {
    /// 角色 ID
    pub role_id: String,
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色权限列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// 角色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 扩展属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

impl UpdateRoleRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }

        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("角色名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("角色名称长度不能超过100个字符".to_string());
            }
        }

        if let Some(ref desc) = self.description {
            if desc.len() > 500 {
                return Err("角色描述长度不能超过500个字符".to_string());
            }
        }

        Ok(())
    }
}

/// 列出角色请求参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesRequest {
    /// 应用 token
    pub app_token: String,
    /// 页面大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 角色类型过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
}

/// 列出角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesResponse {
    /// 角色列表
    pub roles: Vec<Role>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 页面 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// 角色权限定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Permission {
    /// 权限 ID
    pub id: String,
    /// 权限名称
    pub name: String,
    /// 权限描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限类型
    pub permission_type: String,
}