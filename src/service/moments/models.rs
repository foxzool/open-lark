use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============ 帖子相关结构 ============

/// 查询帖子信息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PostGetRequest {
    /// 帖子ID
    pub post_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 帖子信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    /// 帖子ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    /// 发布者用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    /// 发布者姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// 帖子标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 帖子内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 帖子内容类型（text、rich_text等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 媒体附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_list: Option<Vec<PostMedia>>,
    /// 帖子状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 可见性设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<PostVisibility>,
    /// 统计数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<PostStatistics>,
    /// 其他扩展字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, Value>>,
}

/// 帖子媒体附件
#[derive(Debug, Serialize, Deserialize)]
pub struct PostMedia {
    /// 媒体类型（image、video、file等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// 媒体URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// 媒体文件key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_key: Option<String>,
    /// 缩略图URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

/// 帖子可见性设置
#[derive(Debug, Serialize, Deserialize)]
pub struct PostVisibility {
    /// 可见性类型（public、department、custom等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_type: Option<String>,
    /// 可见的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_user_ids: Option<Vec<String>>,
    /// 可见的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_department_ids: Option<Vec<String>>,
}

/// 帖子统计数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PostStatistics {
    /// 评论数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<i64>,
    /// 点赞数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    /// 阅读数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i64>,
    /// 分享数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_count: Option<i64>,
    /// 表情互动统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_stats: Option<HashMap<String, i64>>,
}

// ============ 评论相关结构 ============

/// 评论信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    /// 评论ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    /// 帖子ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    /// 评论者用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    /// 评论者姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// 评论内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 评论内容类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 父评论ID（用于回复）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_comment_id: Option<String>,
    /// 回复的用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_user_id: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 媒体附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_list: Option<Vec<PostMedia>>,
}

// ============ 表情互动相关结构 ============

/// 表情互动信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    /// 互动ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_id: Option<String>,
    /// 帖子ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    /// 评论ID（如果是对评论的互动）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    /// 互动用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 表情类型（like、dislike、heart等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_type: Option<String>,
    /// 表情emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

// ============ 事件相关结构 ============

/// 帖子事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PostEvent {
    /// 事件类型（created、deleted等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 帖子信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Post>,
    /// 事件时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// 操作者用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
}

/// 评论事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentEvent {
    /// 事件类型（created、deleted等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 评论信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    /// 事件时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// 操作者用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
}

/// 表情互动事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionEvent {
    /// 事件类型（created、deleted等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 表情互动信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Reaction>,
    /// 事件时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// 操作者用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
}

/// 帖子统计数据事件
#[derive(Debug, Serialize, Deserialize)]
pub struct PostStatisticsEvent {
    /// 事件类型（updated等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 帖子ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    /// 更新后的统计数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<PostStatistics>,
    /// 统计数据变更详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<StatisticsChanges>,
    /// 事件时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
}

/// 统计数据变更详情
#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsChanges {
    /// 评论数变更
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_count_change: Option<i64>,
    /// 点赞数变更
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count_change: Option<i64>,
    /// 阅读数变更
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count_change: Option<i64>,
    /// 分享数变更
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_count_change: Option<i64>,
    /// 表情互动变更
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_changes: Option<HashMap<String, i64>>,
}

// ============ 内容格式转换相关结构 ============

/// 内容格式转换请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentFormatRequest {
    /// 原始内容
    pub content: String,
    /// 源格式类型
    pub from_format: String,
    /// 目标格式类型
    pub to_format: String,
}

/// 内容格式转换响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentFormatResponse {
    /// 转换后的内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 格式类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_type: Option<String>,
}
