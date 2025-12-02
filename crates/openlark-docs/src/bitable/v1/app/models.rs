//! Bitable V1 应用管理数据模型
//!
//! 定义了应用管理相关的数据结构，包括请求和响应模型。

use serde::{Deserialize, Serialize};

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct App {
    /// 应用token
    pub app_token: String,
    /// 应用名称
    pub name: String,
    /// 应用图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 应用链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 是否被删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// 应用设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_settings: Option<AppSettings>,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSettings {
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// 创建应用请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppRequest {
    /// 应用名称
    pub name: String,
    /// 文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 应用设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_settings: Option<AppSettings>,
}

/// 创建应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppResponse {
    /// 应用信息
    pub app: App,
}

/// 复制应用请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyAppRequest {
    /// 新应用名称
    pub name: String,
    /// 目标文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 是否复制数据表
    pub folder_type: String,
    /// 复制的数据表ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id_list: Option<Vec<String>>,
}

/// 复制应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyAppResponse {
    /// 应用信息
    pub app: App,
}

/// 获取应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAppResponse {
    /// 应用信息
    pub app: App,
}

/// 更新应用请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppRequest {
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 应用图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 应用设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_settings: Option<AppSettings>,
}

/// 更新应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppResponse {
    /// 应用信息
    pub app: App,
}

/// 删除应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAppResponse {
    /// 是否成功
    pub success: bool,
}

impl CreateAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("应用名称不能为空".to_string());
        }

        if self.name.len() > 100 {
            return Err("应用名称长度不能超过100个字符".to_string());
        }

        Ok(())
    }
}

impl CopyAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("新应用名称不能为空".to_string());
        }

        if self.name.len() > 100 {
            return Err("应用名称长度不能超过100个字符".to_string());
        }

        if self.folder_type.trim().is_empty() {
            return Err("文件夹类型不能为空".to_string());
        }

        Ok(())
    }
}

impl UpdateAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("应用名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("应用名称长度不能超过100个字符".to_string());
            }
        }

        Ok(())
    }
}