//! 公司圈（Moments）服务
//!
//! 提供飞书公司圈的完整功能集，支持帖子管理、内容互动、事件处理、
//! 统计分析等企业级社交协作能力。是企业内部沟通和文化建设的重要平台。
//!
//! # 核心功能
//!
//! ## 帖子管理
//! - 📝 帖子内容查询和管理
//! - 📷 多媒体附件支持
//! - 👥 帖子可见性控制
//! - 📊 帖子统计数据获取
//! - 🔍 帖子搜索和筛选
//!
//! ## 内容互动
//! - 👍 表情互动和点赞
//! - 💬 评论发布和管理
//! - 📤 帖子分享和转发
//! - 🔔 互动消息通知
//! - 📈 互动数据统计
//!
//! ## 事件处理
//! - 📮 帖子发布/删除事件
//! - 💬 评论发布/删除事件
//! - 👍 表情互动事件回调
//! - 📊 统计数据变更事件
//! - 🔄 实时事件推送
//!
//! ## 权限管理
//! - 🔐 内容访问权限控制
//! - 👥 用户权限验证
//! - 📊 操作日志记录
//! - 🛡️ 内容安全审核
//! - 🔒 隐私保护机制
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取公司圈服务
//! let moments = &client.moments;
//!
//! // 查询帖子信息
//! // let post_request = GetPostRequest::builder()
//! //     .post_id("post_123")
//! //     .user_id_type("open_id")
//! //     .build();
//! // let post = moments.post.get(post_request, None).await?;
//!
//! // 获取帖子统计数据
//! // let stats_request = GetPostStatsRequest::builder()
//! //     .post_id("post_123")
//! //     .build();
//! // let stats = moments.post.get_stats(stats_request, None).await?;
//!
//! // 处理帖子事件
//! // moments.events.on_post_created(|event| {
//! //     println!("新帖子发布: {:?}", event);
//! // });
//!
//! // 处理互动事件
//! // moments.events.on_reaction_added(|event| {
//! //     println!("收到点赞: {:?}", event);
//! // });
//! ```
//!
//! # 企业社交特性
//!
//! - 🏢 企业内部社交平台
//! - 🎯 精准的内容推荐
//! - 📊 丰富的数据分析
//! - 🔔 智能消息提醒
//! - 📱 多平台同步支持
//!
//! # 文化建设
//!
//! - 🎉 企业文化传播
//! - 👥 团队凝聚力建设
//! - 💡 创新想法分享
//! - 🏆 成就展示平台
//! - 📈 员工参与度提升

/// 事件处理功能
pub mod events;
/// 数据模型定义
pub mod models;
/// 帖子管理功能
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
/// ```ignore
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_moments_service_creation() {
        let config = Config::default();
        let service = MomentsService::new(config.clone());

        assert_eq!(service.post.config.app_id, config.app_id);
        assert_eq!(service.post.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_moments_service_with_custom_config() {
        let config = Config::builder()
            .app_id("moments_test_app")
            .app_secret("moments_test_secret")
            .req_timeout(Duration::from_secs(150))
            .build();

        let service = MomentsService::new(config.clone());

        assert_eq!(service.post.config.app_id, "moments_test_app");
        assert_eq!(service.post.config.app_secret, "moments_test_secret");
        assert_eq!(
            service.post.config.req_timeout,
            Some(Duration::from_secs(150))
        );
    }

    #[test]
    fn test_moments_service_config_independence() {
        let config1 = Config::builder().app_id("moments_app_1").build();

        let config2 = Config::builder().app_id("moments_app_2").build();

        let service1 = MomentsService::new(config1);
        let service2 = MomentsService::new(config2);

        assert_eq!(service1.post.config.app_id, "moments_app_1");
        assert_eq!(service2.post.config.app_id, "moments_app_2");
        assert_ne!(service1.post.config.app_id, service2.post.config.app_id);
    }

    #[test]
    fn test_moments_service_sub_services_accessible() {
        let config = Config::default();
        let service = MomentsService::new(config.clone());

        assert_eq!(service.post.config.app_id, config.app_id);
        // events service doesn't have config, just verify it exists
        let _ = &service.events;
    }

    #[test]
    fn test_moments_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = MomentsService::new(config.clone());

        assert_eq!(service.post.config.app_id, "clone_test_app");
        assert_eq!(service.post.config.app_secret, "clone_test_secret");
    }

    #[test]
    fn test_moments_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = MomentsService::new(config);

        assert_eq!(
            service.post.config.req_timeout,
            Some(Duration::from_secs(180))
        );
    }

    #[test]
    fn test_moments_service_multiple_instances() {
        let config = Config::default();

        let service1 = MomentsService::new(config.clone());
        let service2 = MomentsService::new(config.clone());

        assert_eq!(service1.post.config.app_id, service2.post.config.app_id);
        assert_eq!(
            service1.post.config.app_secret,
            service2.post.config.app_secret
        );
    }

    #[test]
    fn test_moments_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(90))
            .build();

        let service = MomentsService::new(config);

        assert_eq!(service.post.config.app_id, "consistency_test");
        assert_eq!(service.post.config.app_secret, "consistency_secret");
        assert_eq!(
            service.post.config.req_timeout,
            Some(Duration::from_secs(90))
        );
    }
}
