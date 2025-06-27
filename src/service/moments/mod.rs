pub mod events;
pub mod models;
pub mod post;

use crate::core::config::Config;
use events::EventsService;
use post::PostService;

/// 公司圈服务
///
/// 提供飞书公司圈相关的API功能，包括：
/// - 帖子管理：查询帖子信息
/// - 事件处理：帖子、评论、表情互动、统计数据变更事件回调
///
/// # 功能特性
///
/// ## 帖子管理
/// - 查询帖子详细信息，包括内容、作者、媒体附件等
/// - 支持获取帖子统计数据（评论数、点赞数、阅读数等）
/// - 获取帖子可见性设置信息
///
/// ## 事件处理
/// - 帖子发布/删除事件回调
/// - 评论发布/删除事件回调  
/// - 表情互动/取消互动事件回调
/// - 帖子统计数据变更事件回调
/// - 提供事件分发器和处理器接口
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::moments::models::PostGetRequest;
///
/// let client = LarkClient::builder(&app_id, &app_secret).build();
///
/// // 查询帖子信息
/// let request = PostGetRequest {
///     post_id: "post_123".to_string(),
///     user_id_type: Some("open_id".to_string()),
/// };
///
/// let response = client.moments.post.get_post(request, None).await?;
/// ```
pub struct MomentsService {
    /// 帖子管理服务
    pub post: PostService,
    /// 事件处理服务
    pub events: EventsService,
}

impl MomentsService {
    pub fn new(config: Config) -> Self {
        Self {
            post: PostService::new(config),
            events: EventsService::new(),
        }
    }
}
