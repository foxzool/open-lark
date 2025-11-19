//! Bitable Advanced Permission API 数据模型
//!
//! 提供多维表格高级权限管理相关的数据结构，支持V2版本的
//! 角色管理和高级权限控制等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 列出V2角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesV2Request {
    /// 应用token
    pub app_token: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 角色类型过滤（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<RoleTypeV2>,
}

impl ListRolesV2Request {
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

/// 列出V2角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListRolesV2Response {
    /// 角色信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleInfoV2>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListRolesV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建V2角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleV2Request {
    /// 应用token
    pub app_token: String,
    /// 角色名称
    pub name: String,
    /// 角色描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色类型
    pub role_type: RoleTypeV2,
    /// 权限配置
    pub permissions: Vec<PermissionConfig>,
    /// 是否为系统角色（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system: Option<bool>,
}

impl CreateRoleV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.name.trim().is_empty() {
            return Err("角色名称不能为空".to_string());
        }
        if self.name.len() > 100 {
            return Err("角色名称不能超过100个字符".to_string());
        }
        if let Some(ref description) = self.description {
            if description.len() > 1000 {
                return Err("角色描述不能超过1000个字符".to_string());
            }
        }
        if self.permissions.is_empty() {
            return Err("权限配置不能为空".to_string());
        }
        if self.permissions.len() > 100 {
            return Err("权限配置不能超过100个".to_string());
        }
        for (index, permission) in self.permissions.iter().enumerate() {
            if let Err(e) = permission.validate() {
                return Err(format!("权限{}验证失败: {}", index + 1, e));
            }
        }
        Ok(())
    }
}

/// 创建V2角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateRoleV2Response {
    /// 角色信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleInfoV2>,
}

impl ApiResponseTrait for CreateRoleV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新V2角色请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoleV2Request {
    /// 应用token
    pub app_token: String,
    /// 角色ID
    pub role_id: String,
    /// 角色名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 角色描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionConfig>>,
    /// 角色状态（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatusV2>,
}

impl UpdateRoleV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("角色名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("角色名称不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 1000 {
                return Err("角色描述不能超过1000个字符".to_string());
            }
        }
        if let Some(ref permissions) = self.permissions {
            if permissions.is_empty() {
                return Err("权限配置不能为空".to_string());
            }
            if permissions.len() > 100 {
                return Err("权限配置不能超过100个".to_string());
            }
            for (index, permission) in permissions.iter().enumerate() {
                if let Err(e) = permission.validate() {
                    return Err(format!("权限{}验证失败: {}", index + 1, e));
                }
            }
        }
        Ok(())
    }
}

/// 更新V2角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateRoleV2Response {
    /// 角色信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleInfoV2>,
}

impl ApiResponseTrait for UpdateRoleV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// V2角色信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleInfoV2 {
    /// 角色ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<RoleTypeV2>,
    /// 角色状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleStatusV2>,
    /// 权限配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionConfig>>,
    /// 是否为系统角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system: Option<bool>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfoV2>,
}

/// V2角色类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoleTypeV2 {
    /// 管理员角色
    #[serde(rename = "admin")]
    Admin,
    /// 编辑者角色
    #[serde(rename = "editor")]
    Editor,
    /// 查看者角色
    #[serde(rename = "viewer")]
    Viewer,
    /// 评论者角色
    #[serde(rename = "commenter")]
    Commenter,
    /// 自定义角色
    #[serde(rename = "custom")]
    Custom,
}

/// V2角色状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoleStatusV2 {
    /// 启用
    #[serde(rename = "enabled")]
    Enabled,
    /// 禁用
    #[serde(rename = "disabled")]
    Disabled,
}

/// 权限配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PermissionConfig {
    /// 权限类型
    pub permission_type: PermissionType,
    /// 权限范围
    pub scope: PermissionScope,
    /// 权限级别
    pub level: PermissionLevel,
    /// 资源ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl PermissionConfig {
    /// 验证权限配置
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref resource_id) = self.resource_id {
            if resource_id.trim().is_empty() {
                return Err("资源ID不能为空".to_string());
            }
        }
        Ok(())
    }
}

/// 权限类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionType {
    /// 读取权限
    #[serde(rename = "read")]
    Read,
    /// 写入权限
    #[serde(rename = "write")]
    Write,
    /// 删除权限
    #[serde(rename = "delete")]
    Delete,
    /// 分享权限
    #[serde(rename = "share")]
    Share,
    /// 管理权限
    #[serde(rename = "manage")]
    Manage,
    /// 评论权限
    #[serde(rename = "comment")]
    Comment,
}

/// 权限范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionScope {
    /// 应用级别
    #[serde(rename = "app")]
    App,
    /// 表格级别
    #[serde(rename = "table")]
    Table,
    /// 字段级别
    #[serde(rename = "field")]
    Field,
    /// 记录级别
    #[serde(rename = "record")]
    Record,
    /// 视图级别
    #[serde(rename = "view")]
    View,
}

/// 权限级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    /// 完全权限
    #[serde(rename = "full")]
    Full,
    /// 部分权限
    #[serde(rename = "partial")]
    Partial,
    /// 仅自己的数据
    #[serde(rename = "own")]
    Own,
}

/// V2用户信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserInfoV2 {
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
    /// 用户部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
}
