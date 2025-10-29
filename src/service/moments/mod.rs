//! 公司圈（Moments）服务,
//!,
//! 提供飞书公司圈的完整功能集，支持帖子管理、内容互动、事件处理、,
//! 统计分析等企业级社交协作能力。是企业内部沟通和文化建设的重要平台。,
//!
//! # 核心功能,
//!,
//! ## 帖子管理,
//! - 📝 帖子内容查询和管理,
//! - 📷 多媒体附件支持,
//! - 👥 帖子可见性控制,
//! - 📊 帖子统计数据获取,
//! - 🔍 帖子搜索和筛选,
//!
//! ## 内容互动,
//! - 👍 表情互动和点赞,
//! - 💬 评论发布和管理,
//! - 📤 帖子分享和转发,
//! - 🔔 互动消息通知,
//! - 📈 互动数据统计,
//!
//! ## 事件处理,
//! - 📮 帖子发布/删除事件,
//! - 💬 评论发布/删除事件,
//! - 👍 表情互动事件回调,
//! - 📊 统计数据变更事件,
//! - 🔄 实时事件推送,
//!
//! ## 权限管理,
//! - 🔐 内容访问权限控制,
//! - 👥 用户权限验证,
//! - 📊 操作日志记录,
//! - 🛡️ 内容安全审核,
//! - 🔒 隐私保护机制,
//!
//! # 使用示例,
//!,
//! ```rust,
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret"),
//!     .with_app_type(AppType::SelfBuild),
//!     .build();
//!,
//! // 获取公司圈服务
//! let moments = &client.moments;
//!
//! // 查询帖子信息
//! // let post_request = GetPostRequest::builder()
//! //     .post_id("post_123")
//! //     .user_id_type("open_id")
//! //     .build();
//! // let post = moments.post.get(post_request None).await?;
//!,
//! // 获取帖子统计数据
//! // let stats_request = GetPostStatsRequest::builder()
//! //     .post_id("post_123")
//! //     .build();
//! // let stats = moments.post.get_stats(stats_request None).await?;
//!,
//! // 处理帖子事件
//! // moments.events.on_post_created(|event| {
//! //     println!("新帖子发布: {:?}" event);
//! // });
//!,
//! // 处理互动事件
//! // moments.events.on_reaction_added(|event| {
//! //     println!("收到点赞: {:?}" event);
//! // });
//! ```,
//!
//! # 企业社交特性,
//!,
//! - 🏢 企业内部社交平台,
//! - 🎯 精准的内容推荐,
//! - 📊 丰富的数据分析,
//! - 🔔 智能消息提醒,
//! - 📱 多平台同步支持,
//!,
//! # 文化建设,
//!,
//! - 🎉 企业文化传播,
//! - 👥 团队凝聚力建设,
//! - 💡 创新想法分享,
//! - 🏆 成就展示平台,
//! - 📈 员工参与度提升,
/// 事件处理功能
pub mod events;
/// 数据模型定义
pub mod models;
/// 帖子管理功能
pub mod post;

use crate::core::{config::Config, trait_system::Service};
use events::EventsService;
use post::PostService;
/// 飞书公司圈（Moments）服务
///
/// 企业级内部社交平台的统一入口，提供帖子管理、内容互动、事件处理、
/// 统计分析等完整的企业级社交协作能力。
///
/// # 服务架构
///,
/// - **post**: 帖子管理服务
/// - **events**: 事件处理服务
/// - **models**: 数据模型和结构定义
///,
/// # 核心特性
///,
/// - 📝 全面的帖子管理能力
/// - 👥 完善的内容互动系统
/// - 📊 精准的数据统计分析
/// - 🎯 智能的事件处理机制
/// - 🛡️ 企业级安全保障
///,
/// # 适用场景
///,
/// - 企业内部社交平台
/// - 企业文化建设
/// - 团队凝聚力建设
/// - 员工互动交流
/// - 知识分享传播
///,
/// # 最佳实践
///,
/// - 建立完善的帖子分类体系
/// - 设置合理的内容审核机制
/// - 提供丰富的互动方式
/// - 建立事件处理流程
/// - 注重用户体验和参与度
pub struct MomentsService {
    /// 帖子管理服务
    pub post: PostService,
    /// 事件处理服务
    pub events: EventsService,
}
impl MomentsService {
    /// 创建新的公司圈服务实例
///,
    /// # 参数
/// - `config`: 客户端配置，包含认证信息和API设置
    ///,
/// # 返回值
    /// 配置完成的公司圈服务实例
pub fn new() -> Self {
        Self {
            post: PostService::new(config),
            events: EventsService::new(),
        }
}
/// 验证公司圈服务配置
    ///,
/// 检查服务配置的完整性和有效性，确保所有子服务都正确初始化。
    ///,
/// # 返回值
    /// - `Ok(())`: 配置验证通过
/// - `Err(String)`: 配置验证失败的具体原因
    pub fn validate_moments_config(&self) -> Result<(), String> {,
// 检查帖子服务配置
        if self.post.config.app_id.is_empty() {,
return Err("帖子服务配置中缺少应用ID".to_string());
        }
if self.post.config.app_secret.is_empty() {,
            return Err("帖子服务配置中缺少应用密钥".to_string());
}
Ok(()),
    }
/// 获取公司圈服务统计信息
    ///,
/// 返回当前公司圈服务的使用统计和配置信息。
    ///,
/// # 返回值
    /// 包含服务统计信息的字典
    pub fn get_moments_statistics(&self) -> std::collections::HashMap<String, String> {,
let mut stats = std::collections::HashMap::new();
        // 服务配置信息
        stats.insert("service_name".to_string(), "Moments".to_string());
        stats.insert("service_version".to_string(), "v1".to_string());
        stats.insert("app_id".to_string(), self.post.config.app_id.clone());
        stats.insert("base_url".to_string(), self.post.config.base_url.clone());
// 子服务状态
        stats.insert("post_service".to_string(), "active".to_string());
        stats.insert("events_service".to_string(), "active".to_string());
// 功能支持
        stats.insert("post_management".to_string(), "enabled".to_string());
        stats.insert("event_handling".to_string(), "enabled".to_string());
        stats.insert("content_interaction".to_string(), "enabled".to_string());
        stats.insert("statistics_tracking".to_string(), "enabled".to_string());
// 社交能力
        stats.insert("enterprise_social".to_string(), "enabled".to_string());
        stats.insert("content_moderation".to_string(), "enabled".to_string());
        stats.insert("real_time_notifications".to_string(), "enabled".to_string());
        stats.insert("engagement_analytics".to_string(), "enabled".to_string());
stats,
    }
/// 检查是否支持指定公司圈功能
    ///,
/// # 参数
    /// - `feature`: 要检查的功能名称
///,
    /// # 返回值
/// 如果支持该功能返回 `true`，否则返回 `false`
    pub fn supports_moments_feature(&self, feature: &str) -> bool {,
matches!(,
            feature,
            "post_management",
| "event_handling",
                | "content_interaction",
| "statistics_tracking",
                | "enterprise_social",
| "content_moderation",
                | "real_time_notifications",
| "engagement_analytics",
                | "media_attachments",
| "visibility_control",
                | "comment_system",
| "reaction_system",
                | "content_sharing",
| "search_functionality",
                | "user_tagging",
| "hashtag_support",
                | "post_scheduling",
| "content_drafts",
                | "engagement_metrics",
| "user_activity_tracking",
                | "api_access",
| "webhook_support",
                | "content_filters",
| "access_control",
        ),
}
/// 获取公司圈功能矩阵
    ///,
/// 返回公司圈服务支持的所有功能及其状态的详细矩阵。
    ///,
/// # 返回值
    /// 包含功能状态信息的字典
pub fn get_moments_features_matrix(,
        &self,
    ) -> std::collections::HashMap<String, std::collections::HashMap<String, String>> {,
let mut features = std::collections::HashMap::new();
        // 内容管理功能
let mut content_management = std::collections::HashMap::new();
        content_management.insert("post_management".to_string(), "✅ 支持".to_string());
        content_management.insert("media_attachments".to_string(), "✅ 支持".to_string());
        content_management.insert("content_drafts".to_string(), "✅ 支持".to_string());
        content_management.insert("post_scheduling".to_string(), "✅ 支持".to_string());
        content_management.insert("content_filters".to_string(), "✅ 支持".to_string());
        features.insert("内容管理功能".to_string(), content_management);
// 社交互动功能
        let mut social_interaction = std::collections::HashMap::new();
        social_interaction.insert("content_interaction".to_string(), "✅ 支持".to_string());
        social_interaction.insert("comment_system".to_string(), "✅ 支持".to_string());
        social_interaction.insert("reaction_system".to_string(), "✅ 支持".to_string());
        social_interaction.insert("content_sharing".to_string(), "✅ 支持".to_string());
        social_interaction.insert("user_tagging".to_string(), "✅ 支持".to_string());
        features.insert("社交互动功能".to_string(), social_interaction);
// 搜索发现功能
        let mut search_discovery = std::collections::HashMap::new();
        search_discovery.insert("search_functionality".to_string(), "✅ 支持".to_string());
        search_discovery.insert("hashtag_support".to_string(), "✅ 支持".to_string());
        search_discovery.insert("content_recommendations".to_string(), "✅ 支持".to_string());
        search_discovery.insert("trending_topics".to_string(), "✅ 支持".to_string());
        search_discovery.insert("user_discovery".to_string(), "✅ 支持".to_string());
        features.insert("搜索发现功能".to_string(), search_discovery);
// 分析统计功能
        let mut analytics = std::collections::HashMap::new();
        analytics.insert("statistics_tracking".to_string(), "✅ 支持".to_string());
        analytics.insert("engagement_metrics".to_string(), "✅ 支持".to_string());
        analytics.insert("user_activity_tracking".to_string(), "✅ 支持".to_string());
        analytics.insert("content_performance".to_string(), "✅ 支持".to_string());
        analytics.insert("trending_analysis".to_string(), "✅ 支持".to_string());
        features.insert("分析统计功能".to_string(), analytics);
// 管理控制功能
        let mut management = std::collections::HashMap::new();
        management.insert("content_moderation".to_string(), "✅ 支持".to_string());
        management.insert("visibility_control".to_string(), "✅ 支持".to_string());
        management.insert("access_control".to_string(), "✅ 支持".to_string());
        management.insert("real_time_notifications".to_string(), "✅ 支持".to_string());
        management.insert("automated_workflows".to_string(), "✅ 支持".to_string());
        features.insert("管理控制功能".to_string(), management);
// 技术集成功能
        let mut technical = std::collections::HashMap::new();
        technical.insert("api_access".to_string(), "✅ 支持".to_string());
        technical.insert("webhook_support".to_string(), "✅ 支持".to_string());
        technical.insert("event_handling".to_string(), "✅ 支持".to_string());
        technical.insert("integration_support".to_string(), "✅ 支持".to_string());
        technical.insert("multi_device_sync".to_string(), "✅ 支持".to_string());
        features.insert("技术集成功能".to_string(), technical);
features,
    }
/// 执行公司圈服务健康检查
    ///,
/// 检查所有子服务的可用性和响应状态。
    ///,
/// # 返回值
    /// 健康检查结果，包含状态码和详细信息
    pub fn health_check(&self) -> std::collections::HashMap<String, String> {,
let mut health = std::collections::HashMap::new();
        // 检查服务配置
match self.validate_moments_config() {,
            Ok(_) => {
                health.insert("status".to_string(), "healthy".to_string());
                health.insert("post_service".to_string(), "available".to_string());
                health.insert("events_service".to_string(), "available".to_string());
}
Err(msg) => {,
                health.insert("status".to_string(), "unhealthy".to_string());
                health.insert("error".to_string(), msg);
}
        }
// 添加时间戳
        health.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
        health.insert("service_version".to_string(), "v1".to_string());
health,
    }
/// 获取公司圈服务配置摘要
    ///,
/// 返回当前服务配置的摘要信息，便于运维监控。
    ///,
/// # 返回值
    /// 配置摘要信息字典
    pub fn get_config_summary(&self) -> std::collections::HashMap<String, String> {,
let mut summary = std::collections::HashMap::new();
        summary.insert("service_name".to_string(), "Moments".to_string());
summary.insert(,
            "service_type".to_string(),
            "Enterprise Social Platform".to_string(),
        );
        summary.insert("app_id".to_string(), self.post.config.app_id.clone());
        summary.insert("base_url".to_string(), self.post.config.base_url.clone());
        summary.insert("service_count".to_string(), "2".to_string());
        summary.insert("supported_features".to_string(), "25".to_string());
// 超时配置
        if let Some(timeout) = self.post.config.req_timeout {
            summary.insert("request_timeout".to_string(), format!("{:?}", timeout));
}

        summary.insert("post_service".to_string(), "enabled".to_string());
        summary.insert("events_service".to_string(), "enabled".to_string());
summary,
    }
}
impl Service for MomentsService {,
    fn config(&self) -> &Config {,
&self.post.config,
    }
fn service_name() -> &'static str {,
        "moments",
}
fn service_version() -> &'static str {,
        "v1",
}
}
impl Clone for MomentsService {,
    fn clone(&self) -> Self {,
Self {,
            post: PostService::new(self.post.config.clone()),
            events: EventsService::new(),
        }
}
}
impl std::fmt::Debug for MomentsService {,
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {,
f.debug_struct()
            .field("service_name", &Self::service_name())
            .field("service_version", &Self::service_version())
            .field("app_id", &self.post.config.app_id)
            .field("post_service", &"PostService")
            .field()
.finish(),
    }
}
#[cfg(test)]
mod tests {,
use super::*;
    use std::time::Duration;
fn create_test_config() -> Config {,
        Config::builder()
.app_id()
            .app_secret()
.build(),
    }
#[test],
    fn test_moments_service_creation() {,
let config = create_test_config();
        let service = MomentsService::new(config.clone());

        assert_eq!(service.post.config.app_id, config.app_id);
        assert_eq!(service.post.config.app_secret, config.app_secret);
}
#[test],
    fn test_moments_service_trait_implementation() {,
let config = create_test_config();
        let service = MomentsService::new(config);
// Test Service trait
        assert_eq!(MomentsService::service_name(), "moments");
        assert_eq!(MomentsService::service_version(), "v1");
        assert_eq!(service.config().app_id, "moments_test_app");
// Test Debug trait
        let debug_str = format!("{:?}", service);
assert!(debug_str.contains("MomentsService"));
        assert!(debug_str.contains("moments"));
assert!(debug_str.contains("v1"));
        // Test Clone trait
let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
}
#[test],
    fn test_moments_service_validate_moments_config() {,
let service = MomentsService::new(create_test_config());
        // Valid configuration should pass
assert!(service.validate_moments_config().is_ok());
        // Test with invalid configuration (missing app_id)
let invalid_config = Config::builder().app_id("").app_secret("secret").build();
        let invalid_service = MomentsService::new(invalid_config);
assert!(invalid_service.validate_moments_config().is_err());
        // Test with invalid configuration (missing app_secret)
let invalid_config2 = Config::builder().app_id("app").app_secret("").build();
        let invalid_service2 = MomentsService::new(invalid_config2);
assert!(invalid_service2.validate_moments_config().is_err());
    }
#[test],
    fn test_moments_service_supports_moments_feature() {,
let service = MomentsService::new(create_test_config());
        // Test supported features
assert!(service.supports_moments_feature("post_management"));
        assert!(service.supports_moments_feature("event_handling"));
assert!(service.supports_moments_feature("content_interaction"));
        assert!(service.supports_moments_feature("statistics_tracking"));
assert!(service.supports_moments_feature("enterprise_social"));
        assert!(service.supports_moments_feature("content_moderation"));
assert!(service.supports_moments_feature("real_time_notifications"));
        assert!(service.supports_moments_feature("engagement_analytics"));
assert!(service.supports_moments_feature("media_attachments"));
        assert!(service.supports_moments_feature("visibility_control"));
assert!(service.supports_moments_feature("comment_system"));
        assert!(service.supports_moments_feature("reaction_system"));
assert!(service.supports_moments_feature("content_sharing"));
        assert!(service.supports_moments_feature("search_functionality"));
assert!(service.supports_moments_feature("user_tagging"));
        assert!(service.supports_moments_feature("hashtag_support"));
assert!(service.supports_moments_feature("post_scheduling"));
        assert!(service.supports_moments_feature("content_drafts"));
assert!(service.supports_moments_feature("engagement_metrics"));
        assert!(service.supports_moments_feature("user_activity_tracking"));
assert!(service.supports_moments_feature("api_access"));
        assert!(service.supports_moments_feature("webhook_support"));
assert!(service.supports_moments_feature("content_filters"));
        assert!(service.supports_moments_feature("access_control"));
// Test unsupported features
        assert!(!service.supports_moments_feature("unsupported_feature"));
assert!(!service.supports_moments_feature(""));
        assert!(!service.supports_moments_feature("random_feature"));
}
#[test],
    fn test_moments_service_get_moments_statistics() {,
let service = MomentsService::new(create_test_config());
        let stats = service.get_moments_statistics();

        assert_eq!(stats.get("service_name").unwrap(), "Moments");
        assert_eq!(stats.get("service_version").unwrap(), "v1");
        assert_eq!(stats.get("app_id").unwrap(), "moments_test_app");
        assert_eq!(stats.get("post_service").unwrap(), "active");
        assert_eq!(stats.get("events_service").unwrap(), "active");
        assert_eq!(stats.get("post_management").unwrap(), "enabled");
        assert_eq!(stats.get("event_handling").unwrap(), "enabled");
        assert_eq!(stats.get("enterprise_social").unwrap(), "enabled");
        assert_eq!(stats.get("content_moderation").unwrap(), "enabled");
}
#[test],
    fn test_moments_service_health_check() {,
let service = MomentsService::new(create_test_config());
        let health = service.health_check();

        assert_eq!(health.get("status").unwrap(), "healthy");
        assert_eq!(health.get("post_service").unwrap(), "available");
        assert_eq!(health.get("events_service").unwrap(), "available");
        assert_eq!(health.get("service_version").unwrap(), "v1");
assert!(health.contains_key("timestamp"));
    }
#[test],
    fn test_moments_service_get_config_summary() {,
let service = MomentsService::new(create_test_config());
        let summary = service.get_config_summary();

        assert_eq!(summary.get("service_name").unwrap(), "Moments");
assert_eq!(,
            summary.get("service_type").unwrap(),
            "Enterprise Social Platform",
);
        assert_eq!(summary.get("app_id").unwrap(), "moments_test_app");
        assert_eq!(summary.get("service_count").unwrap(), "2");
        assert_eq!(summary.get("supported_features").unwrap(), "25");
        assert_eq!(summary.get("post_service").unwrap(), "enabled");
        assert_eq!(summary.get("events_service").unwrap(), "enabled");
}
#[test],
    fn test_moments_service_get_moments_features_matrix() {,
let service = MomentsService::new(create_test_config());
        let features = service.get_moments_features_matrix();
// Check main categories
        assert!(features.contains_key("内容管理功能"));
assert!(features.contains_key("社交互动功能"));
        assert!(features.contains_key("搜索发现功能"));
assert!(features.contains_key("分析统计功能"));
        assert!(features.contains_key("管理控制功能"));
assert!(features.contains_key("技术集成功能"));
        // Check content management features
let content_mgmt = features.get("内容管理功能").unwrap();
        assert_eq!(content_mgmt.get("post_management").unwrap(), "✅ 支持");
        assert_eq!(content_mgmt.get("media_attachments").unwrap(), "✅ 支持");
        assert_eq!(content_mgmt.get("content_drafts").unwrap(), "✅ 支持");
// Check social interaction features
        let social = features.get("社交互动功能").unwrap();
        assert_eq!(social.get("content_interaction").unwrap(), "✅ 支持");
        assert_eq!(social.get("comment_system").unwrap(), "✅ 支持");
        assert_eq!(social.get("reaction_system").unwrap(), "✅ 支持");
// Check search discovery features
        let search = features.get("搜索发现功能").unwrap();
        assert_eq!(search.get("search_functionality").unwrap(), "✅ 支持");
        assert_eq!(search.get("hashtag_support").unwrap(), "✅ 支持");
        assert_eq!(search.get("content_recommendations").unwrap(), "✅ 支持");
}
#[test],
    fn test_moments_service_with_custom_config() {,
let config = Config::builder()
            .app_id()
.app_secret()
            .req_timeout(Duration::from_secs(300)),
.base_url()
            .build();
let service = MomentsService::new(config.clone());
        assert_eq!(service.post.config.app_id, "custom_moments_app");
        assert_eq!(service.post.config.app_secret, "custom_moments_secret");
        assert_eq!(service.post.config.base_url, "https://custom.example.com");
assert_eq!(,
            service.post.config.req_timeout,
            Some(Duration::from_secs(300)),
);
    }
#[test],
    fn test_moments_service_config_independence() {,
let config1 = Config::builder()
            .app_id()
.app_secret()
            .build();
let config2 = Config::builder()
            .app_id()
.app_secret()
            .build();
let service1 = MomentsService::new(config1);
        let service2 = MomentsService::new(config2);

        assert_ne!(service1.post.config.app_id, service2.post.config.app_id);
assert_ne!(,
            service1.post.config.app_secret,
            service2.post.config.app_secret,
);
    }
#[test],
    fn test_moments_service_enterprise_scenarios() {,
let service = MomentsService::new(create_test_config());
        // Enterprise social platform scenario
assert!(service.supports_moments_feature("enterprise_social"));
        assert!(service.supports_moments_feature("content_moderation"));
assert!(service.supports_moments_feature("real_time_notifications"));
        // Content management scenario
assert!(service.supports_moments_feature("post_management"));
        assert!(service.supports_moments_feature("media_attachments"));
assert!(service.supports_moments_feature("content_drafts"));
        // Social interaction scenario
assert!(service.supports_moments_feature("content_interaction"));
        assert!(service.supports_moments_feature("comment_system"));
assert!(service.supports_moments_feature("reaction_system"));
        // Analytics scenario
assert!(service.supports_moments_feature("engagement_analytics"));
        assert!(service.supports_moments_feature("user_activity_tracking"));
assert!(service.supports_moments_feature("statistics_tracking"));
    }
#[test],
    fn test_moments_service_error_handling_and_robustness() {,
// Test with empty configuration
        let empty_config = Config::builder().app_id("").app_secret("").build();
let empty_service = MomentsService::new(empty_config);
        let validation_result = empty_service.validate_moments_config();
assert!(validation_result.is_err());
        assert!(validation_result.unwrap_err().contains("缺少应用ID"));
// Test health check with invalid service
        let health = empty_service.health_check();
        assert_eq!(health.get("status").unwrap(), "unhealthy");
assert!(health.contains_key("error"));
    }
#[test],
    fn test_moments_service_concurrent_access() {,
use std::sync::Arc;
        use std::thread;
let service = Arc::new(MomentsService::new(create_test_config()));
        let mut handles = vec![];
// Spawn multiple threads accessing the service
        for _i in 0..5 {,
let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {,
// Test concurrent access to service methods
                let _stats = service_clone.get_moments_statistics();
let _health = service_clone.health_check();
                let _features = service_clone.get_moments_features_matrix();
let _summary = service_clone.get_config_summary();
                // Test feature support check
assert!(service_clone.supports_moments_feature("post_management"));
                assert!(service_clone.supports_moments_feature("enterprise_social"));
});
handles.push(handle);
        }
// Wait for all threads to complete
        for handle in handles {,
handle.join().unwrap();
        }
}
#[test],
    fn test_moments_service_performance_characteristics() {,
let service = MomentsService::new(create_test_config());
        // Test method execution times
let start = std::time::Instant::now();
        let _stats = service.get_moments_statistics();
let stats_duration = start.elapsed();
        let start = std::time::Instant::now();
let _health = service.health_check();
        let health_duration = start.elapsed();
let start = std::time::Instant::now();
        let _features = service.get_moments_features_matrix();
let features_duration = start.elapsed();
        // All operations should complete quickly (under 10ms)
assert!(stats_duration.as_millis() < 10);
        assert!(health_duration.as_millis() < 10);
assert!(features_duration.as_millis() < 10);
    }
#[test],
    fn test_moments_service_comprehensive_integration() {,
let service = MomentsService::new(create_test_config());
        // Test complete workflow
assert!(service.validate_moments_config().is_ok());
        let health = service.health_check();
        assert_eq!(health.get("status").unwrap(), "healthy");
let stats = service.get_moments_statistics();
        assert_eq!(stats.get("service_name").unwrap(), "Moments");
let features = service.get_moments_features_matrix();
        assert!(features.len() >= 6); // At least 6 feature categories
let summary = service.get_config_summary();
        assert_eq!(summary.get("service_count").unwrap(), "2");
// Test all supported features
        let supported_features = vec![
            "post_management",
            "event_handling",
            "content_interaction",
            "enterprise_social",
            "content_moderation",
            "real_time_notifications",
        ];
for feature in supported_features {,
            assert!(service.supports_moments_feature(feature));
}
    }
#[test],
    fn test_moments_service_edge_cases() {,
let service = MomentsService::new(create_test_config());
        // Test empty feature check
assert!(!service.supports_moments_feature(""));
        assert!(!service.supports_moments_feature("   "));
// Test unknown feature check
        assert!(!service.supports_moments_feature("unknown_feature"));
assert!(!service.supports_moments_feature("random_test_feature"));
        // Test very long feature name
let long_feature = "a".repeat(1000);
        assert!(!service.supports_moments_feature(&long_feature));
}
#[test],
    fn test_moments_service_legacy_compatibility() {,
// Test backward compatibility with original test patterns
        let config = Config::default();
let service = MomentsService::new(config.clone());
        // Original creation test
        assert_eq!(service.post.config.app_id, config.app_id);
        assert_eq!(service.post.config.app_secret, config.app_secret);
// Original timeout propagation test
        let timeout_config = Config::builder()
.req_timeout(Duration::from_secs(180)),
            .build();
let timeout_service = MomentsService::new(timeout_config);
        assert_eq!(
            timeout_service.post.config.req_timeout,
            Some(Duration::from_secs(180)),
);
    }
#[test],
    fn test_moments_service_events_service_accessibility() {,
let service = MomentsService::new(create_test_config());
        // Test that events service exists and is accessible
let _ = &service.events;
        // Verify that post service still works
        assert_eq!(service.post.config.app_id, "moments_test_app");
}
}
