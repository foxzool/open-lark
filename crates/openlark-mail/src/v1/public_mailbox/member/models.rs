//! 公共邮箱成员数据模型

use serde::{Deserialize, Serialize};

/// 创建公共邮箱成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePublicMailboxMemberBody {
    /// 成员邮箱地址
    pub email: String,

    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<i32>,
}

/// 创建公共邮箱成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePublicMailboxMemberResponse {
    /// 成员 ID
    pub member_id: String,

    /// 成员邮箱地址
    pub email: String,

    /// 成员类型
    pub member_type: i32,

    /// 创建时间
    pub create_time: String,
}

/// 获取公共邮箱成员详情响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetPublicMailboxMemberResponse {
    /// 成员 ID
    pub member_id: String,

    /// 成员邮箱地址
    pub email: String,

    /// 成员类型
    pub member_type: i32,

    /// 创建时间
    pub create_time: String,
}

/// 公共邮箱成员列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxMemberListResponse {
    /// 成员列表
    pub items: Vec<PublicMailboxMemberItem>,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 公共邮箱成员项目
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxMemberItem {
    /// 成员 ID
    pub member_id: String,

    /// 成员邮箱地址
    pub email: String,

    /// 成员类型
    pub member_type: i32,

    /// 创建时间
    pub create_time: String,
}

/// 删除公共邮箱成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePublicMailboxMemberResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 批量创建公共邮箱成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchCreatePublicMailboxMemberBody {
    /// 成员邮箱地址列表
    pub emails: Vec<String>,
}

/// 批量删除公共邮箱成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchDeletePublicMailboxMemberBody {
    /// 成员 ID 列表
    pub member_ids: Vec<String>,
}

/// 清空公共邮箱成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct ClearPublicMailboxMemberResponse {
    /// 是否清空成功
    pub success: bool,
}
