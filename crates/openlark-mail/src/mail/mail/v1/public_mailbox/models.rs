//! 公共邮箱模块数据模型

use serde::{Deserialize, Serialize};

/// 创建公共邮箱请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePublicMailboxBody {
    /// 公共邮箱名称
    pub name: String,

    /// 公共邮箱描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
}

/// 创建公共邮箱响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePublicMailboxResponse {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 公共邮箱名称
    pub name: String,

    /// 公共邮箱描述
    #[serde(default)]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(default)]
    pub owner_user_id: Option<String>,

    /// 是否已删除
    pub is_deleted: bool,

    /// 创建时间
    pub created_time: String,

    /// 更新时间
    #[serde(default)]
    pub update_time: Option<String>,
}

/// 获取公共邮箱详情响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetPublicMailboxResponse {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 公共邮箱名称
    pub name: String,

    /// 公共邮箱描述
    #[serde(default)]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(default)]
    pub owner_user_id: Option<String>,

    /// 是否已删除
    pub is_deleted: bool,

    /// 创建时间
    pub created_time: String,

    /// 更新时间
    #[serde(default)]
    pub update_time: Option<String>,
}

/// 更新公共邮箱请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdatePublicMailboxBody {
    /// 公共邮箱名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 公共邮箱描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
}

/// 更新公共邮箱响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePublicMailboxResponse {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 更新时间
    pub update_time: String,
}

/// 部分更新公共邮箱请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct PatchPublicMailboxBody {
    /// 公共邮箱名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 公共邮箱描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 部分更新公共邮箱响应
#[derive(Debug, Clone, Deserialize)]
pub struct PatchPublicMailboxResponse {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 更新时间
    pub update_time: String,
}

/// 删除公共邮箱响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePublicMailboxResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 公共邮箱列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxListResponse {
    /// 公共邮箱列表
    pub items: Vec<PublicMailboxItem>,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 公共邮箱项目
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxItem {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 公共邮箱名称
    pub name: String,

    /// 公共邮箱描述
    #[serde(default)]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(default)]
    pub owner_user_id: Option<String>,

    /// 是否已删除
    pub is_deleted: bool,

    /// 创建时间
    pub created_time: String,
}

/// 移至回收站响应
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveToRecycleBinResponse {
    /// 是否成功
    pub success: bool,
}
