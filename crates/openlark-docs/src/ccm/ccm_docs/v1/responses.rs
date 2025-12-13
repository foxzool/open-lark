/// CCM Docs V1 响应类型定义
use serde::{Deserialize, Serialize};

// Re-export types from models
pub use super::models::{DocumentItem, MetaItem, PermissionInfo, UserInfo};

/// 搜索云文档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectData {
    /// 文档列表
    pub items: Vec<DocumentItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 页面标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

/// 元数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    /// 文档信息列表
    pub items: Vec<MetaItem>,
    /// 总数
    pub total: Option<i32>,
}