//! Bitable App API 数据模型
//!
//! 提供多维表格应用管理相关的数据结构，支持应用创建、复制、
//! 元数据管理和基础信息操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建多维表格请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppRequest {
    /// 多维表格名称
    pub name: String,
    /// 文件夹token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 是否复制模板（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_template_token: Option<String>,
}

impl CreateAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("多维表格名称不能为空".to_string());
        }
        if self.name.len() > 100 {
            return Err("多维表格名称不能超过100个字符".to_string());
        }
        Ok(())
    }
}

/// 创建多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateAppResponse {
    /// 应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppInfo>,
}

impl ApiResponseTrait for CreateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制多维表格请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyAppRequest {
    /// 目标文件夹token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 复制后的名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CopyAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("复制后的名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("复制后的名称不能超过100个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 复制多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CopyAppResponse {
    /// 复制后的应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppInfo>,
}

impl ApiResponseTrait for CopyAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取多维表格元数据请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAppRequest {
    /// 应用token
    pub app_token: String,
}

impl GetAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取多维表格元数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetAppResponse {
    /// 应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppInfo>,
}

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新多维表格元数据请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppRequest {
    /// 应用token
    pub app_token: String,
    /// 应用名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 应用图标（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 应用描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("应用名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("应用名称不能超过100个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 更新多维表格元数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateAppResponse {
    /// 应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppInfo>,
}

impl ApiResponseTrait for UpdateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AppInfo {
    /// 应用token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_token: Option<String>,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 应用图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 应用链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 应用描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    /// 是否开启高级权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
    /// 时间区域
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserInfo>,
    /// 是否被删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

/// 用户信息
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

/// 删除多维表格请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAppRequest {
    /// 应用token
    pub app_token: String,
}

impl DeleteAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteAppResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
