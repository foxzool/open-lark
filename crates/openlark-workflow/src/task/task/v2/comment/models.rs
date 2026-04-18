//! 评论 API v2 的数据模型（strict 路径）

use serde::{Deserialize, Serialize};

/// 评论成员信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommentMember {
    /// 成员 ID。
    pub id: String,
    /// 成员类型。
    #[serde(rename = "type")]
    pub member_type: String,
    /// 成员角色。
    pub role: String,
}

/// 评论详情。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommentItem {
    /// 评论 ID。
    pub id: String,
    /// 评论内容。
    pub content: String,
    /// 评论创建人。
    pub creator: CommentMember,
    /// 被回复的评论 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_comment_id: Option<String>,
    /// 评论创建时间戳（ms）。
    pub created_at: String,
    /// 评论更新时间戳（ms）。
    pub updated_at: String,
    /// 关联资源类型。
    pub resource_type: String,
    /// 关联资源 ID。
    pub resource_id: String,
}

/// 获取评论详情响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetCommentResponse {
    /// 评论详情。
    pub comment: CommentItem,
}

/// 获取评论列表响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListCommentsResponse {
    /// 评论列表。
    #[serde(default)]
    pub items: Vec<CommentItem>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: bool,
}
