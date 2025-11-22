//! Bitable App Table View API 数据模型
//!
//! 提供多维表格视图管理相关的数据结构，支持视图的创建、
//! 查询、更新、删除等操作。

use serde_json::Value;
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建视图请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateViewRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: String,
    /// 视图配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_config: Option<Value>,
    /// 视图描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateViewRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.view_name.trim().is_empty() {
            return Err("视图名称不能为空".to_string());
        }
        if self.view_name.len() > 100 {
            return Err("视图名称不能超过100个字符".to_string());
        }
        if self.view_type.trim().is_empty() {
            return Err("视图类型不能为空".to_string());
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("视图描述不能超过500个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 创建视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateViewResponse {
    /// 视图信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<ViewInfo>,
}

impl ApiResponseTrait for CreateViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新视图请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateViewRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 视图ID
    pub view_id: String,
    /// 视图名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    /// 视图配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_config: Option<Value>,
    /// 视图描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateViewRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.view_id.trim().is_empty() {
            return Err("视图ID不能为空".to_string());
        }
        if let Some(ref view_name) = self.view_name {
            if view_name.trim().is_empty() {
                return Err("视图名称不能为空".to_string());
            }
            if view_name.len() > 100 {
                return Err("视图名称不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("视图描述不能超过500个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 更新视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateViewResponse {
    /// 视图信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<ViewInfo>,
}

impl ApiResponseTrait for UpdateViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除视图请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteViewRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 视图ID
    pub view_id: String,
}

impl DeleteViewRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.view_id.trim().is_empty() {
            return Err("视图ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteViewResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出视图请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListViewsRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListViewsRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }
        Ok(())
    }
}

/// 列出视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListViewsResponse {
    /// 视图信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub views: Option<Vec<ViewInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取视图请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetViewRequest {
    /// 应用token
    pub app_token: String,
    /// 数据表ID
    pub table_id: String,
    /// 视图ID
    pub view_id: String,
}

impl GetViewRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.view_id.trim().is_empty() {
            return Err("视图ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetViewResponse {
    /// 视图信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<ViewInfo>,
}

impl ApiResponseTrait for GetViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 视图信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ViewInfo {
    /// 视图ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    /// 视图名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    /// 视图类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
    /// 视图配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_config: Option<Value>,
    /// 视图描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 视图序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 是否可见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
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

/// 视图类型常量
pub mod view_types {
    /// 表格视图
    pub const GRID: &str = "grid";
    /// 看板视图
    pub const KANBAN: &str = "kanban";
    /// 甘特图视图
    pub const GANTT: &str = "gantt";
    /// 日历视图
    pub const CALENDAR: &str = "calendar";
    /// 画廊视图
    pub const GALLERY: &str = "gallery";
    /// 表单视图
    pub const FORM: &str = "form";
    /// 画册视图
    pub const ALBUM: &str = "album";
    /// 列表视图
    pub const LIST: &str = "list";
}
