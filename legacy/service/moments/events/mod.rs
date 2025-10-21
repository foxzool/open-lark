use serde::{Deserialize, Serialize};

use crate::service::moments::models::{
    CommentEvent, PostEvent, PostStatisticsEvent, ReactionEvent,
};

/// 公司圈事件处理服务
pub struct EventsService {
    // 事件处理服务不需要配置，主要用于定义事件处理器
}

impl EventsService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EventsService {
    fn default() -> Self {
        Self::new()
    }
}

/// 帖子事件处理器特征
pub trait PostEventHandler {
    /// 处理帖子发布事件
    fn handle_post_created(&self, event: PostEvent);

    /// 处理帖子删除事件
    fn handle_post_deleted(&self, event: PostEvent);
}

/// 评论事件处理器特征
pub trait CommentEventHandler {
    /// 处理评论发布事件
    fn handle_comment_created(&self, event: CommentEvent);

    /// 处理评论删除事件
    fn handle_comment_deleted(&self, event: CommentEvent);
}

/// 表情互动事件处理器特征
pub trait ReactionEventHandler {
    /// 处理表情互动事件
    fn handle_reaction_created(&self, event: ReactionEvent);

    /// 处理取消表情互动事件
    fn handle_reaction_deleted(&self, event: ReactionEvent);
}

/// 帖子统计数据事件处理器特征
pub trait PostStatisticsEventHandler {
    /// 处理帖子统计数据变更事件
    fn handle_post_statistics_updated(&self, event: PostStatisticsEvent);
}

/// 组合事件处理器 - 实现所有事件类型的处理
pub trait MomentsEventHandler:
    PostEventHandler + CommentEventHandler + ReactionEventHandler + PostStatisticsEventHandler
{
    /// 获取事件处理器名称
    fn get_handler_name(&self) -> &str {
        "MomentsEventHandler"
    }
}

/// 事件类型枚举
#[derive(Debug, Serialize, Deserialize)]
pub enum MomentsEventType {
    /// 帖子发布
    PostCreated,
    /// 帖子删除
    PostDeleted,
    /// 评论发布
    CommentCreated,
    /// 评论删除
    CommentDeleted,
    /// 表情互动
    ReactionCreated,
    /// 取消表情互动
    ReactionDeleted,
    /// 帖子统计数据变更
    PostStatisticsUpdated,
}

/// 通用事件包装器
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum MomentsEvent {
    /// 帖子事件
    #[serde(rename = "post_created")]
    PostCreated { event: PostEvent },
    #[serde(rename = "post_deleted")]
    PostDeleted { event: PostEvent },

    /// 评论事件
    #[serde(rename = "comment_created")]
    CommentCreated { event: CommentEvent },
    #[serde(rename = "comment_deleted")]
    CommentDeleted { event: CommentEvent },

    /// 表情互动事件
    #[serde(rename = "reaction_created")]
    ReactionCreated { event: ReactionEvent },
    #[serde(rename = "reaction_deleted")]
    ReactionDeleted { event: ReactionEvent },

    /// 帖子统计数据事件
    #[serde(rename = "post_statistics_updated")]
    PostStatisticsUpdated { event: PostStatisticsEvent },
}

/// 事件分发器
pub struct MomentsEventDispatcher<H>
where
    H: MomentsEventHandler,
{
    handler: H,
}

impl<H> MomentsEventDispatcher<H>
where
    H: MomentsEventHandler,
{
    /// 创建新的事件分发器
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    /// 分发事件到对应的处理器
    pub fn dispatch_event(&self, event: MomentsEvent) {
        match event {
            MomentsEvent::PostCreated { event } => {
                self.handler.handle_post_created(event);
            }
            MomentsEvent::PostDeleted { event } => {
                self.handler.handle_post_deleted(event);
            }
            MomentsEvent::CommentCreated { event } => {
                self.handler.handle_comment_created(event);
            }
            MomentsEvent::CommentDeleted { event } => {
                self.handler.handle_comment_deleted(event);
            }
            MomentsEvent::ReactionCreated { event } => {
                self.handler.handle_reaction_created(event);
            }
            MomentsEvent::ReactionDeleted { event } => {
                self.handler.handle_reaction_deleted(event);
            }
            MomentsEvent::PostStatisticsUpdated { event } => {
                self.handler.handle_post_statistics_updated(event);
            }
        }
    }

    /// 获取处理器引用
    pub fn get_handler(&self) -> &H {
        &self.handler
    }
}

/// 默认事件处理器实现 - 提供基础的日志记录
pub struct DefaultMomentsEventHandler {
    pub name: String,
}

impl DefaultMomentsEventHandler {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl PostEventHandler for DefaultMomentsEventHandler {
    fn handle_post_created(&self, event: PostEvent) {
        log::info!(
            "[{}] 处理帖子发布事件: post_id={:?}, author_id={:?}",
            self.name,
            event.post.as_ref().and_then(|p| p.post_id.as_ref()),
            event.post.as_ref().and_then(|p| p.author_id.as_ref())
        );
    }

    fn handle_post_deleted(&self, event: PostEvent) {
        log::info!(
            "[{}] 处理帖子删除事件: post_id={:?}, operator_id={:?}",
            self.name,
            event.post.as_ref().and_then(|p| p.post_id.as_ref()),
            event.operator_id
        );
    }
}

impl CommentEventHandler for DefaultMomentsEventHandler {
    fn handle_comment_created(&self, event: CommentEvent) {
        log::info!(
            "[{}] 处理评论发布事件: comment_id={:?}, post_id={:?}",
            self.name,
            event.comment.as_ref().and_then(|c| c.comment_id.as_ref()),
            event.comment.as_ref().and_then(|c| c.post_id.as_ref())
        );
    }

    fn handle_comment_deleted(&self, event: CommentEvent) {
        log::info!(
            "[{}] 处理评论删除事件: comment_id={:?}, operator_id={:?}",
            self.name,
            event.comment.as_ref().and_then(|c| c.comment_id.as_ref()),
            event.operator_id
        );
    }
}

impl ReactionEventHandler for DefaultMomentsEventHandler {
    fn handle_reaction_created(&self, event: ReactionEvent) {
        log::info!(
            "[{}] 处理表情互动事件: reaction_type={:?}, post_id={:?}",
            self.name,
            event
                .reaction
                .as_ref()
                .and_then(|r| r.reaction_type.as_ref()),
            event.reaction.as_ref().and_then(|r| r.post_id.as_ref())
        );
    }

    fn handle_reaction_deleted(&self, event: ReactionEvent) {
        log::info!(
            "[{}] 处理取消表情互动事件: reaction_type={:?}, post_id={:?}",
            self.name,
            event
                .reaction
                .as_ref()
                .and_then(|r| r.reaction_type.as_ref()),
            event.reaction.as_ref().and_then(|r| r.post_id.as_ref())
        );
    }
}

impl PostStatisticsEventHandler for DefaultMomentsEventHandler {
    fn handle_post_statistics_updated(&self, event: PostStatisticsEvent) {
        log::info!(
            "[{}] 处理帖子统计数据变更事件: post_id={:?}",
            self.name,
            event.post_id
        );
    }
}

impl MomentsEventHandler for DefaultMomentsEventHandler {
    fn get_handler_name(&self) -> &str {
        &self.name
    }
}
