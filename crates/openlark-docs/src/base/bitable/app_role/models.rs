//! Bitable App Role API 数据模型
//!
//! 提供多维表格角色权限管理相关的数据结构，支持角色的创建、
//! 查询、更新、删除等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleRequest {
    /// 应用token
    pub app_token: String,
    /// 角色名称
    pub role_name: String,
    /// 角色类型
    pub role_type: String,
    /// 角色描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionConfig>>,
}

impl CreateRoleRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_name.trim().is_empty() {
            return Err("角色名称不能为空".to_string());
        }
        if self.role_name.len() > 100 {
            return Err("角色名称不能超过100个字符".to_string());
        }
        if self.role_type.trim().is_empty() {
            return Err("角色类型不能为空".to_string());
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("角色描述不能超过500个字符".to_string());
            }
        }
        if let Some(ref permissions) = self.permissions {
            if permissions.len() > 100 {
                return Err("权限配置数量不能超过100个".to_string());
            }
        }
        Ok(())
    }
}

/// 创建角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateRoleResponse {
    /// 角色信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleInfo>,
}

impl ApiResponseTrait for CreateRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoleRequest {
    /// 应用token
    pub app_token: String,
    /// 角色ID
    pub role_id: String,
    /// 角色名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 角色描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionConfig>>,
}

impl UpdateRoleRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }
        if let Some(ref role_name) = self.role_name {
            if role_name.trim().is_empty() {
                return Err("角色名称不能为空".to_string());
            }
            if role_name.len() > 100 {
                return Err("角色名称不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("角色描述不能超过500个字符".to_string());
            }
        }
        if let Some(ref permissions) = self.permissions {
            if permissions.len() > 100 {
                return Err("权限配置数量不能超过100个".to_string());
            }
        }
        Ok(())
    }
}

/// 更新角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateRoleResponse {
    /// 角色信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleInfo>,
}

impl ApiResponseTrait for UpdateRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRoleRequest {
    /// 应用token
    pub app_token: String,
    /// 角色ID
    pub role_id: String,
}

impl DeleteRoleRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteRoleResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesRequest {
    /// 应用token
    pub app_token: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListRolesRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 列出角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListRolesResponse {
    /// 角色信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListRolesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleInfo {
    /// 角色ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 角色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionConfig>>,
    /// 角色成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
}

/// 权限配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PermissionConfig {
    /// 权限类型
    pub permission_type: String,
    /// 权限值
    pub permission_value: String,
    /// 权限范围（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// 用户信息（重用自其他模块）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserInfo {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// 角色类型常量
pub mod role_types {
    /// 所有者角色
    pub const OWNER: &str = "owner";
    /// 管理员角色
    pub const ADMIN: &str = "admin";
    /// 编辑者角色
    pub const EDITOR: &str = "editor";
    /// 评论者角色
    pub const COMMENTER: &str = "commenter";
    /// 查看者角色
    pub const VIEWER: &str = "viewer";
    /// 自定义角色
    pub const CUSTOM: &str = "custom";
}

/// 权限类型常量
pub mod permission_types {
    /// 应用权限
    pub const APP: &str = "app";
    /// 表格权限
    pub const TABLE: &str = "table";
    /// 字段权限
    pub const FIELD: &str = "field";
    /// 记录权限
    pub const RECORD: &str = "record";
    /// 视图权限
    pub const VIEW: &str = "view";
}

/// 权限值常量
pub mod permission_values {
    /// 管理权限
    pub const MANAGE: &str = "manage";
    /// 编辑权限
    pub const EDIT: &str = "edit";
    /// 评论权限
    pub const COMMENT: &str = "comment";
    /// 查看权限
    pub const VIEW: &str = "view";
    /// 无权限
    pub const NONE: &str = "none";
}
