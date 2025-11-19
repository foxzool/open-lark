//! Bitable App Role Member API 数据模型
//!
//! 提供多维表格角色成员管理相关的数据结构，支持角色成员的添加、
//! 查询、删除等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleMemberRequest {
    /// 应用token
    pub app_token: String,
    /// 角色ID
    pub role_id: String,
    /// 用户ID
    pub user_id: String,
}

impl CreateRoleMemberRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }
        if self.user_id.trim().is_empty() {
            return Err("用户ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 创建角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateRoleMemberResponse {
    /// 成员信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<RoleMemberInfo>,
}

impl ApiResponseTrait for CreateRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRoleMemberRequest {
    /// 应用token
    pub app_token: String,
    /// 角色ID
    pub role_id: String,
    /// 成员ID
    pub member_id: String,
}

impl DeleteRoleMemberRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }
        if self.member_id.trim().is_empty() {
            return Err("成员ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteRoleMemberResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRoleMembersRequest {
    /// 应用token
    pub app_token: String,
    /// 角色ID
    pub role_id: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListRoleMembersRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 列出角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListRoleMembersResponse {
    /// 成员信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RoleMemberInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListRoleMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量创建角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRoleMemberRequest {
    /// 应用token
    pub app_token: String,
    /// 角色ID
    pub role_id: String,
    /// 成员列表
    pub members: Vec<CreateRoleMemberRequest>,
}

impl BatchCreateRoleMemberRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }
        if self.members.is_empty() {
            return Err("成员列表不能为空".to_string());
        }
        if self.members.len() > 100 {
            return Err("单次批量添加成员数量不能超过100个".to_string());
        }
        for (i, member) in self.members.iter().enumerate() {
            member
                .validate()
                .map_err(|e| format!("第{}个成员验证失败: {}", i + 1, e))?;
        }
        Ok(())
    }
}

/// 批量创建角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchCreateRoleMemberResponse {
    /// 成员信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RoleMemberInfo>>,
}

impl ApiResponseTrait for BatchCreateRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRoleMemberRequest {
    /// 应用token
    pub app_token: String,
    /// 角色ID
    pub role_id: String,
    /// 成员ID列表
    pub member_ids: Vec<String>,
}

impl BatchDeleteRoleMemberRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.role_id.trim().is_empty() {
            return Err("角色ID不能为空".to_string());
        }
        if self.member_ids.is_empty() {
            return Err("成员ID列表不能为空".to_string());
        }
        if self.member_ids.len() > 100 {
            return Err("单次批量删除成员数量不能超过100个".to_string());
        }
        Ok(())
    }
}

/// 批量删除角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchDeleteRoleMemberResponse {
    /// 删除结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DeleteMemberResult>>,
}

impl ApiResponseTrait for BatchDeleteRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 角色成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleMemberInfo {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
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
    /// 角色ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 添加时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 添加者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
}

/// 删除成员结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteMemberResult {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
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
