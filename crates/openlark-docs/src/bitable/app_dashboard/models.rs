//! Bitable App Dashboard API 数据模型
//!
//! 提供多维表格仪表板管理相关的数据结构，支持仪表板的创建、
//! 查询、更新、删除等操作。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 创建仪表板请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateDashboardRequest {
    /// 应用token
    pub app_token: String,
    /// 仪表板名称
    pub dashboard_name: String,
    /// 仪表板描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 仪表板布局配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_config: Option<Value>,
    /// 仪表板组件配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<DashboardComponent>>,
}

impl CreateDashboardRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.dashboard_name.trim().is_empty() {
            return Err("仪表板名称不能为空".to_string());
        }
        if self.dashboard_name.len() > 100 {
            return Err("仪表板名称不能超过100个字符".to_string());
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("仪表板描述不能超过500个字符".to_string());
            }
        }
        if let Some(ref components) = self.components {
            if components.len() > 50 {
                return Err("组件数量不能超过50个".to_string());
            }
        }
        Ok(())
    }
}

/// 创建仪表板响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateDashboardResponse {
    /// 仪表板信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<DashboardInfo>,
}

impl ApiResponseTrait for CreateDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新仪表板请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateDashboardRequest {
    /// 应用token
    pub app_token: String,
    /// 仪表板ID
    pub dashboard_id: String,
    /// 仪表板名称（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<String>,
    /// 仪表板描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 仪表板布局配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_config: Option<Value>,
    /// 仪表板组件配置（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<DashboardComponent>>,
}

impl UpdateDashboardRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.dashboard_id.trim().is_empty() {
            return Err("仪表板ID不能为空".to_string());
        }
        if let Some(ref dashboard_name) = self.dashboard_name {
            if dashboard_name.trim().is_empty() {
                return Err("仪表板名称不能为空".to_string());
            }
            if dashboard_name.len() > 100 {
                return Err("仪表板名称不能超过100个字符".to_string());
            }
        }
        if let Some(ref description) = self.description {
            if description.len() > 500 {
                return Err("仪表板描述不能超过500个字符".to_string());
            }
        }
        if let Some(ref components) = self.components {
            if components.len() > 50 {
                return Err("组件数量不能超过50个".to_string());
            }
        }
        Ok(())
    }
}

/// 更新仪表板响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateDashboardResponse {
    /// 仪表板信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<DashboardInfo>,
}

impl ApiResponseTrait for UpdateDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除仪表板请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteDashboardRequest {
    /// 应用token
    pub app_token: String,
    /// 仪表板ID
    pub dashboard_id: String,
}

impl DeleteDashboardRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.dashboard_id.trim().is_empty() {
            return Err("仪表板ID不能为空".to_string());
        }
        Ok(())
    }
}

/// 删除仪表板响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteDashboardResponse {
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出仪表板请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListDashboardsRequest {
    /// 应用token
    pub app_token: String,
    /// 页面大小（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListDashboardsRequest {
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

/// 列出仪表板响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListDashboardsResponse {
    /// 仪表板信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<DashboardInfo>>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多页面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListDashboardsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制仪表板请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyDashboardRequest {
    /// 应用token
    pub app_token: String,
    /// 仪表板ID
    pub dashboard_id: String,
    /// 新仪表板名称
    pub dashboard_name: String,
}

impl CopyDashboardRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.dashboard_id.trim().is_empty() {
            return Err("仪表板ID不能为空".to_string());
        }
        if self.dashboard_name.trim().is_empty() {
            return Err("新仪表板名称不能为空".to_string());
        }
        if self.dashboard_name.len() > 100 {
            return Err("仪表板名称不能超过100个字符".to_string());
        }
        Ok(())
    }
}

/// 复制仪表板响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CopyDashboardResponse {
    /// 仪表板信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<DashboardInfo>,
}

impl ApiResponseTrait for CopyDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 仪表板信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DashboardInfo {
    /// 仪表板ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    /// 仪表板名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<String>,
    /// 仪表板描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 仪表板布局配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_config: Option<Value>,
    /// 仪表板组件配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<DashboardComponent>>,
    /// 仪表板序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
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

/// 仪表板组件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardComponent {
    /// 组件ID
    pub component_id: String,
    /// 组件类型
    pub component_type: String,
    /// 组件名称
    pub component_name: String,
    /// 组件配置
    pub config: Value,
    /// 组件位置
    pub position: ComponentPosition,
}

/// 组件位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComponentPosition {
    /// X坐标
    pub x: i32,
    /// Y坐标
    pub y: i32,
    /// 宽度
    pub width: i32,
    /// 高度
    pub height: i32,
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

/// 组件类型常量
pub mod component_types {
    /// 数字卡片
    pub const NUMBER_CARD: &str = "number_card";
    /// 图表组件
    pub const CHART: &str = "chart";
    /// 表格组件
    pub const TABLE: &str = "table";
    /// 文本组件
    pub const TEXT: &str = "text";
    /// 图片组件
    pub const IMAGE: &str = "image";
    /// 按钮组件
    pub const BUTTON: &str = "button";
}
