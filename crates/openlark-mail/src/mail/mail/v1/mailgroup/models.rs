//! 邮件 API v1 的数据模型

use serde::{Deserialize, Serialize};

/// 创建邮件组请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateMailGroupBody {
    /// 邮件组名称（邮箱地址）
    pub mail_group_id: String,

    /// 邮件组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 邮件组所有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    /// 邮件组成员邮箱列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,

    /// 仅管理员发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_admins_send: Option<bool>,
}

/// 创建邮件组响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateMailGroupResponse {
    /// 邮件组 ID
    pub mail_group_id: String,

    /// 邮件组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 创建时间
    pub created_at: String,
}

/// 获取邮件组详情响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetMailGroupResponse {
    /// 邮件组 ID
    pub mail_group_id: String,

    /// 邮件组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 邮件组所有者
    #[serde(default)]
    pub owner: Option<String>,

    /// 仅管理员发送
    #[serde(default)]
    pub only_admins_send: Option<bool>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// 更新邮件组请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateMailGroupBody {
    /// 邮件组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 仅管理员发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_admins_send: Option<bool>,
}

/// 更新邮件组响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMailGroupResponse {
    /// 邮件组 ID
    pub mail_group_id: String,

    /// 更新时间
    pub updated_at: String,
}

/// 删除邮件组响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMailGroupResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 邮件组列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct MailGroupListResponse {
    /// 邮件组列表
    pub mail_groups: Vec<MailGroupItem>,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 邮件组项目
#[derive(Debug, Clone, Deserialize)]
pub struct MailGroupItem {
    /// 邮件组 ID
    pub mail_group_id: String,

    /// 邮件组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 创建时间
    pub created_at: String,
}
