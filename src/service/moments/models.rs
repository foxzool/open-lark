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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_post_get_request() {
        let request = PostGetRequest {
            post_id: "post123".to_string(),
            user_id_type: Some("user_id".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("post123"));
        assert!(json.contains("user_id"));
    }

    #[test]
    fn test_post_media() {
        let media = PostMedia {
            media_type: Some("image".to_string()),
            media_url: Some("https://example.com/image.jpg".to_string()),
            media_key: Some("media_key123".to_string()),
            thumbnail_url: Some("https://example.com/thumb.jpg".to_string()),
            file_size: Some(1024000),
            file_name: Some("image.jpg".to_string()),
        };

        let json = serde_json::to_string(&media).unwrap();
        assert!(json.contains("\"image\""));
        assert!(json.contains("https://example.com/image.jpg"));
        assert!(json.contains("media_key123"));
        assert!(json.contains("1024000"));
        assert!(json.contains("image.jpg"));
    }

    #[test]
    fn test_post_visibility() {
        let visibility = PostVisibility {
            visibility_type: Some("custom".to_string()),
            visible_user_ids: Some(vec!["user1".to_string(), "user2".to_string()]),
            visible_department_ids: Some(vec!["dept1".to_string()]),
        };

        let json = serde_json::to_string(&visibility).unwrap();
        assert!(json.contains("\"custom\""));
        assert!(json.contains("user1"));
        assert!(json.contains("user2"));
        assert!(json.contains("dept1"));
    }

    #[test]
    fn test_post_statistics() {
        let mut reaction_stats = HashMap::new();
        reaction_stats.insert("like".to_string(), 25);
        reaction_stats.insert("heart".to_string(), 10);

        let stats = PostStatistics {
            comment_count: Some(15),
            like_count: Some(25),
            view_count: Some(200),
            share_count: Some(5),
            reaction_stats: Some(reaction_stats),
        };

        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("\"comment_count\":15"));
        assert!(json.contains("\"like_count\":25"));
        assert!(json.contains("\"view_count\":200"));
        assert!(json.contains("\"share_count\":5"));
        assert!(json.contains("\"like\":25"));
        assert!(json.contains("\"heart\":10"));
    }

    #[test]
    fn test_post_full() {
        let media = PostMedia {
            media_type: Some("image".to_string()),
            media_url: Some("https://example.com/image.jpg".to_string()),
            media_key: Some("media_key123".to_string()),
            thumbnail_url: None,
            file_size: Some(500000),
            file_name: Some("company_event.jpg".to_string()),
        };

        let visibility = PostVisibility {
            visibility_type: Some("department".to_string()),
            visible_user_ids: None,
            visible_department_ids: Some(vec!["tech_dept".to_string()]),
        };

        let stats = PostStatistics {
            comment_count: Some(5),
            like_count: Some(12),
            view_count: Some(89),
            share_count: Some(2),
            reaction_stats: None,
        };

        let mut extra = HashMap::new();
        extra.insert("tags".to_string(), serde_json::json!(["团建", "技术"]));

        let post = Post {
            post_id: Some("post456".to_string()),
            author_id: Some("ou_author123".to_string()),
            author_name: Some("张三".to_string()),
            title: Some("公司团建活动".to_string()),
            content: Some("今天的团建活动很精彩！".to_string()),
            content_type: Some("rich_text".to_string()),
            media_list: Some(vec![media]),
            status: Some("published".to_string()),
            create_time: Some("2024-01-01T10:00:00Z".to_string()),
            update_time: Some("2024-01-01T10:05:00Z".to_string()),
            visibility: Some(visibility),
            statistics: Some(stats),
            extra: Some(extra),
        };

        let json = serde_json::to_string(&post).unwrap();
        assert!(json.contains("post456"));
        assert!(json.contains("公司团建活动"));
        assert!(json.contains("今天的团建活动很精彩！"));
        assert!(json.contains("ou_author123"));
        assert!(json.contains("张三"));
        assert!(json.contains("published"));
        assert!(json.contains("department"));
        assert!(json.contains("tech_dept"));
        assert!(json.contains("团建"));
    }

    #[test]
    fn test_comment_full() {
        let media = PostMedia {
            media_type: Some("image".to_string()),
            media_url: Some("https://example.com/comment_img.jpg".to_string()),
            media_key: Some("comment_media123".to_string()),
            thumbnail_url: None,
            file_size: Some(200000),
            file_name: Some("response.jpg".to_string()),
        };

        let comment = Comment {
            comment_id: Some("comment789".to_string()),
            post_id: Some("post456".to_string()),
            author_id: Some("ou_commenter456".to_string()),
            author_name: Some("李四".to_string()),
            content: Some("太棒了！期待下次活动".to_string()),
            content_type: Some("text".to_string()),
            parent_comment_id: None,
            reply_to_user_id: None,
            create_time: Some("2024-01-01T11:00:00Z".to_string()),
            update_time: Some("2024-01-01T11:00:00Z".to_string()),
            media_list: Some(vec![media]),
        };

        let json = serde_json::to_string(&comment).unwrap();
        assert!(json.contains("comment789"));
        assert!(json.contains("post456"));
        assert!(json.contains("ou_commenter456"));
        assert!(json.contains("李四"));
        assert!(json.contains("太棒了！期待下次活动"));
        assert!(json.contains("comment_media123"));
    }

    #[test]
    fn test_comment_reply() {
        let reply = Comment {
            comment_id: Some("reply123".to_string()),
            post_id: Some("post456".to_string()),
            author_id: Some("ou_replier789".to_string()),
            author_name: Some("王五".to_string()),
            content: Some("我也是这样认为的！".to_string()),
            content_type: Some("text".to_string()),
            parent_comment_id: Some("comment789".to_string()),
            reply_to_user_id: Some("ou_commenter456".to_string()),
            create_time: Some("2024-01-01T11:30:00Z".to_string()),
            update_time: None,
            media_list: None,
        };

        let json = serde_json::to_string(&reply).unwrap();
        assert!(json.contains("reply123"));
        assert!(json.contains("comment789"));
        assert!(json.contains("ou_replier789"));
        assert!(json.contains("王五"));
        assert!(json.contains("我也是这样认为的！"));
        assert!(json.contains("ou_commenter456"));
        assert!(!json.contains("update_time"));
        assert!(!json.contains("media_list"));
    }

    #[test]
    fn test_reaction() {
        let reaction = Reaction {
            reaction_id: Some("reaction456".to_string()),
            post_id: Some("post456".to_string()),
            comment_id: None,
            user_id: Some("ou_reactor123".to_string()),
            user_name: Some("赵六".to_string()),
            reaction_type: Some("like".to_string()),
            emoji: Some("👍".to_string()),
            create_time: Some("2024-01-01T12:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&reaction).unwrap();
        assert!(json.contains("reaction456"));
        assert!(json.contains("post456"));
        assert!(json.contains("ou_reactor123"));
        assert!(json.contains("赵六"));
        assert!(json.contains("\"like\""));
        assert!(json.contains("👍"));
        assert!(!json.contains("comment_id"));
    }

    #[test]
    fn test_reaction_on_comment() {
        let comment_reaction = Reaction {
            reaction_id: Some("reaction789".to_string()),
            post_id: Some("post456".to_string()),
            comment_id: Some("comment789".to_string()),
            user_id: Some("ou_reactor456".to_string()),
            user_name: Some("孙七".to_string()),
            reaction_type: Some("heart".to_string()),
            emoji: Some("❤️".to_string()),
            create_time: Some("2024-01-01T12:15:00Z".to_string()),
        };

        let json = serde_json::to_string(&comment_reaction).unwrap();
        assert!(json.contains("reaction789"));
        assert!(json.contains("comment789"));
        assert!(json.contains("孙七"));
        assert!(json.contains("\"heart\""));
        assert!(json.contains("❤️"));
    }

    #[test]
    fn test_post_event() {
        let post = Post {
            post_id: Some("post789".to_string()),
            author_id: Some("ou_author456".to_string()),
            author_name: Some("作者".to_string()),
            title: Some("新帖子".to_string()),
            content: Some("这是一个新帖子".to_string()),
            content_type: Some("text".to_string()),
            media_list: None,
            status: Some("published".to_string()),
            create_time: Some("2024-01-01T14:00:00Z".to_string()),
            update_time: None,
            visibility: None,
            statistics: None,
            extra: None,
        };

        let event = PostEvent {
            event_type: Some("created".to_string()),
            post: Some(post),
            event_time: Some("2024-01-01T14:00:01Z".to_string()),
            operator_id: Some("ou_author456".to_string()),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"created\""));
        assert!(json.contains("post789"));
        assert!(json.contains("新帖子"));
        assert!(json.contains("这是一个新帖子"));
        assert!(json.contains("2024-01-01T14:00:01Z"));
    }

    #[test]
    fn test_comment_event() {
        let comment = Comment {
            comment_id: Some("comment456".to_string()),
            post_id: Some("post123".to_string()),
            author_id: Some("ou_commenter123".to_string()),
            author_name: Some("评论者".to_string()),
            content: Some("很好的帖子！".to_string()),
            content_type: Some("text".to_string()),
            parent_comment_id: None,
            reply_to_user_id: None,
            create_time: Some("2024-01-01T15:00:00Z".to_string()),
            update_time: None,
            media_list: None,
        };

        let event = CommentEvent {
            event_type: Some("created".to_string()),
            comment: Some(comment),
            event_time: Some("2024-01-01T15:00:01Z".to_string()),
            operator_id: Some("ou_commenter123".to_string()),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"created\""));
        assert!(json.contains("comment456"));
        assert!(json.contains("很好的帖子！"));
        assert!(json.contains("ou_commenter123"));
    }

    #[test]
    fn test_reaction_event() {
        let reaction = Reaction {
            reaction_id: Some("reaction123".to_string()),
            post_id: Some("post123".to_string()),
            comment_id: None,
            user_id: Some("ou_reactor789".to_string()),
            user_name: Some("互动者".to_string()),
            reaction_type: Some("like".to_string()),
            emoji: Some("👍".to_string()),
            create_time: Some("2024-01-01T16:00:00Z".to_string()),
        };

        let event = ReactionEvent {
            event_type: Some("created".to_string()),
            reaction: Some(reaction),
            event_time: Some("2024-01-01T16:00:01Z".to_string()),
            operator_id: Some("ou_reactor789".to_string()),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"created\""));
        assert!(json.contains("reaction123"));
        assert!(json.contains("互动者"));
        assert!(json.contains("👍"));
    }

    #[test]
    fn test_statistics_changes() {
        let mut reaction_changes = HashMap::new();
        reaction_changes.insert("like".to_string(), 5);
        reaction_changes.insert("heart".to_string(), 2);

        let changes = StatisticsChanges {
            comment_count_change: Some(2),
            like_count_change: Some(5),
            view_count_change: Some(15),
            share_count_change: Some(1),
            reaction_changes: Some(reaction_changes),
        };

        let json = serde_json::to_string(&changes).unwrap();
        assert!(json.contains("\"comment_count_change\":2"));
        assert!(json.contains("\"like_count_change\":5"));
        assert!(json.contains("\"view_count_change\":15"));
        assert!(json.contains("\"share_count_change\":1"));
        assert!(json.contains("\"like\":5"));
        assert!(json.contains("\"heart\":2"));
    }

    #[test]
    fn test_post_statistics_event() {
        let stats = PostStatistics {
            comment_count: Some(20),
            like_count: Some(35),
            view_count: Some(250),
            share_count: Some(8),
            reaction_stats: None,
        };

        let changes = StatisticsChanges {
            comment_count_change: Some(1),
            like_count_change: Some(2),
            view_count_change: Some(10),
            share_count_change: Some(0),
            reaction_changes: None,
        };

        let event = PostStatisticsEvent {
            event_type: Some("updated".to_string()),
            post_id: Some("post123".to_string()),
            statistics: Some(stats),
            changes: Some(changes),
            event_time: Some("2024-01-01T17:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"updated\""));
        assert!(json.contains("post123"));
        assert!(json.contains("\"comment_count\":20"));
        assert!(json.contains("\"like_count\":35"));
        assert!(json.contains("\"comment_count_change\":1"));
        assert!(json.contains("\"like_count_change\":2"));
    }

    #[test]
    fn test_content_format_request() {
        let request = ContentFormatRequest {
            content: "# 标题\n\n这是**粗体**文本".to_string(),
            from_format: "markdown".to_string(),
            to_format: "rich_text".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("# 标题"));
        assert!(json.contains("**粗体**"));
        assert!(json.contains("markdown"));
        assert!(json.contains("rich_text"));
    }

    #[test]
    fn test_content_format_response() {
        let response = ContentFormatResponse {
            content: Some("<h1>标题</h1><p>这是<strong>粗体</strong>文本</p>".to_string()),
            format_type: Some("rich_text".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("<h1>标题</h1>"));
        assert!(json.contains("<strong>粗体</strong>"));
        assert!(json.contains("rich_text"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_post = Post {
            post_id: Some("minimal_post".to_string()),
            author_id: None,
            author_name: None,
            title: None,
            content: None,
            content_type: None,
            media_list: None,
            status: None,
            create_time: None,
            update_time: None,
            visibility: None,
            statistics: None,
            extra: None,
        };

        let json = serde_json::to_string(&minimal_post).unwrap();
        assert!(json.contains("minimal_post"));
        assert!(!json.contains("author_id"));
        assert!(!json.contains("content"));
        assert!(!json.contains("statistics"));

        let minimal_comment = Comment {
            comment_id: Some("minimal_comment".to_string()),
            post_id: None,
            author_id: None,
            author_name: None,
            content: None,
            content_type: None,
            parent_comment_id: None,
            reply_to_user_id: None,
            create_time: None,
            update_time: None,
            media_list: None,
        };

        let comment_json = serde_json::to_string(&minimal_comment).unwrap();
        assert!(comment_json.contains("minimal_comment"));
        assert!(!comment_json.contains("post_id"));
        assert!(!comment_json.contains("content"));
    }
}
