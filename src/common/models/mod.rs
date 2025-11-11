//! 跨服务共享的数据模型
//!
//! 定义了多个服务模块都会用到的通用数据结构，
//! 包括用户信息、分页、错误处理等基础模型。

use api_resp::ApiResponseTrait;
use serde::{Deserialize, Serialize};

/// 用户标识符类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UserIdType {
    /// 开放平台用户ID
    #[serde(rename = "open_id")]
    OpenId(String),
    /// 应用内用户ID
    #[serde(rename = "user_id")]
    UserId(String),
    /// 租户用户ID
    #[serde(rename = "union_id")]
    UnionId(String),
}

impl UserIdType {
    /// 获取用户ID值
    pub fn as_str(&self) -> &str {
        match self {
            UserIdType::OpenId(s) => s,
            UserIdType::UserId(s) => s,
            UserIdType::UnionId(s) => s,
        }
    }
}

/// 基础用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseUser {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// 用户状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    /// 启用
    Enabled,
    /// 禁用
    Disabled,
    /// 未激活
    Unactivated,
    /// 已离职
    Left,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    /// 页面令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 是否还有更多
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl PageInfo {
    /// 创建默认分页信息
    pub fn default() -> Self {
        Self {
            page_token: None,
            page_size: Some(20),
            total: None,
            has_more: Some(false),
        }
    }

    /// 设置分页大小
    pub fn with_page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页面令牌
    pub fn with_page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }
}

/// 基础分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    /// 分页信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<T>>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 时间范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// 地理位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 省份
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 经纬度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// 标签信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签键
    pub key: String,
    /// 标签值
    pub value: String,
}

/// 附件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// 文件key
    pub file_key: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub size: i64,
    /// 文件类型
    pub file_type: String,
    /// 下载URL
    pub url: Option<String>,
}

/// 扩展字段
pub type ExtraFields = std::collections::HashMap<String, serde_json::Value>;

/// 实现ApiResponseTrait的空响应结构体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyResponse;

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> api_resp::ResponseFormat {
        api_resp::ResponseFormat::Data
    }
}
