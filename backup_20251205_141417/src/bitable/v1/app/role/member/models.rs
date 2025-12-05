//! Bitable V1 角色成员管理数据模型
//!
//! 定义了角色成员管理相关的数据结构，包括请求和响应模型。

use serde::{Deserialize, Serialize};

/// 批量新增角色成员请求模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRoleMemberRequestModel {
    /// 协作者列表
    pub member_list: Vec<BatchCreateMemberItemModel>,
}

/// 批量新增成员项模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateMemberItemModel {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 协作者类型
    pub member_type: String,
    /// 权限列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// 批量新增角色成员响应模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRoleMemberResponseModel {
    /// 批量操作结果
    pub results: Vec<BatchCreateResultItemModel>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 页面 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量新增结果项模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateResultItemModel {
    /// 用户ID
    pub user_id: String,
    /// 操作结果
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// 协作者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<RoleMemberInfoModel>,
}

/// 批量删除角色成员请求模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRoleMemberRequestModel {
    /// 协作者ID列表
    pub member_ids: Vec<String>,
}

/// 批量删除角色成员响应模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRoleMemberResponseModel {
    /// 批量操作结果
    pub results: Vec<BatchDeleteResultItemModel>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 页面 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量删除结果项模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteResultItemModel {
    /// 协作者ID
    pub member_id: String,
    /// 操作结果
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 角色成员信息模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMemberInfoModel {
    /// 协作者ID
    pub member_id: String,
    /// 协作者类型
    pub member_type: String,
    /// 用户ID
    pub user_id: String,
    /// 协作者姓名
    pub name: String,
    /// 协作者邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 协作者头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 权限列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// 是否为管理员
    pub is_admin: bool,
    /// 添加时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

impl BatchCreateRoleMemberRequestModel {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.member_list.is_empty() {
            return Err("协作者列表不能为空".to_string());
        }

        for (index, member) in self.member_list.iter().enumerate() {
            if member.user_ids.is_empty() {
                return Err(format!("第{}个协作者的用户ID列表不能为空", index + 1));
            }

            if member.member_type.trim().is_empty() {
                return Err(format!("第{}个协作者的类型不能为空", index + 1));
            }
        }

        Ok(())
    }
}

impl BatchDeleteRoleMemberRequestModel {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.member_ids.is_empty() {
            return Err("协作者ID列表不能为空".to_string());
        }

        if self.member_ids.len() > 500 {
            return Err("批量删除协作者数量不能超过500个".to_string());
        }

        Ok(())
    }
}