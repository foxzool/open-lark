//! 公共邮箱别名数据模型

use serde::{Deserialize, Serialize};

/// 创建公共邮箱别名请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePublicMailboxAliasBody {
    /// 别名邮箱地址
    pub email: String,
}

/// 创建公共邮箱别名响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePublicMailboxAliasResponse {
    /// 别名 ID
    pub alias_id: String,

    /// 别名邮箱地址
    pub email: String,

    /// 创建时间
    pub create_time: String,
}

/// 公共邮箱别名列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxAliasListResponse {
    /// 别名列表
    pub items: Vec<PublicMailboxAliasItem>,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 公共邮箱别名项目
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxAliasItem {
    /// 别名 ID
    pub alias_id: String,

    /// 别名邮箱地址
    pub email: String,

    /// 创建时间
    pub create_time: String,
}

/// 删除公共邮箱别名响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePublicMailboxAliasResponse {
    /// 是否删除成功
    pub success: bool,
}
