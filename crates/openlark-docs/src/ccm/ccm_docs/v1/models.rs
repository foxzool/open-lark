//! CCM Docs V1 数据模型

use serde::{Deserialize, Serialize};

/// 搜索云文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectParams {
    /// 搜索关键字
    pub query: String,
    /// 搜索类型：doc, sheet, bitable, mindnote, file, folder
    #[serde(rename = "doc_type")]
    pub doc_type: Option<String>,
    /// 搜索范围：all, owned, shared
    pub scope: Option<String>,
    /// 每页数量，默认20，最大100
    #[serde(rename = "page_size")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 排序字段：create_time, update_time, title
    #[serde(rename = "sort_field")]
    pub sort_field: Option<String>,
    /// 排序方向：asc, desc
    #[serde(rename = "sort_direction")]
    pub sort_direction: Option<String>,
}

/// 搜索云文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectResponse {
    /// 搜索结果
    pub data: Option<SearchData>,
}

/// 搜索数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchData {
    /// 文档列表
    pub items: Vec<DocumentItem>,
    /// 是否有更多数据
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页面标记
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
}

/// 文档项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentItem {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 文档URL
    pub url: String,
}

/// 获取元数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaParams {
    /// 文档token列表
    #[serde(rename = "obj_tokens")]
    pub obj_tokens: Vec<String>,
    /// 返回类型
    #[serde(rename = "return_type")]
    pub return_type: Option<String>,
}

/// 获取元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaResponse {
    /// 元数据结果
    pub data: Option<MetaData>,
}

/// 元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    /// 文档信息列表
    pub items: Vec<MetaItem>,
}

/// 元数据项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaItem {
    /// 文档token
    #[serde(rename = "obj_token")]
    pub obj_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 所有者信息
    pub owner: Option<UserInfo>,
    /// 权限信息
    pub permission: Option<PermissionInfo>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "open_id")]
    pub open_id: String,
    /// 用户名称
    pub name: String,
}

/// 权限信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionInfo {
    /// 是否可读
    pub readable: bool,
    /// 是否可写
    pub writable: bool,
    /// 是否可分享
    pub shareable: bool,
}