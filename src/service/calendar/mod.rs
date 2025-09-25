//! 日历（Calendar）服务
//!
//! 提供飞书日历的完整功能集，支持日历管理、日程安排、会议室预订等
//! 企业级时间管理需求。是团队协作和时间规划的核心服务模块。
//!
//! # 核心功能
//!
//! ## 日历管理
//! - 📅 个人和共享日历的创建与管理
//! - 🎨 日历外观和属性设置
//! - 👥 日历共享和权限控制
//! - 🔄 日历同步和导入导出
//!
//! ## 日程管理
//! - 📝 日程的增删改查操作
//! - ⏰ 提醒和通知设置
//! - 🔄 重复日程和规则配置
//! - 📋 日程详情和描述管理
//!
//! ## 会议室管理
//! - 🏢 会议室信息和资源管理
//! - 📅 会议室预订和冲突检测
//! - 📊 会议室使用统计和分析
//! - 🔧 会议室设备和设施配置
//!
//! ## 参与人管理
//! - 👤 日程参与者的邀请和管理
//! - ✅ 参与状态跟踪（接受/拒绝/待定）
//! - 📧 邀请通知和提醒发送
//! - 🔄 参与者权限和角色设置
//!
//! ## 高级功能
//! - 🏖️ 请假日程和假期管理
//! - 📝 会议纪要和记录关联
//! - 🔗 CalDAV标准协议支持
//! - 📧 Exchange日历集成和同步
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
//! // 获取日历服务
//! let calendar = &client.calendar;
//!
//! // 创建日历示例
//! // let create_calendar_req = CreateCalendarRequest::builder()
//! //     .summary("团队日历")
//! //     .description("团队日程安排和会议")
//! //     .build();
//! // let calendar_result = calendar.v4.calendar.create(create_calendar_req, None).await?;
//!
//! // 创建日程示例
//! // let create_event_req = CreateEventRequest::builder()
//! //     .calendar_id("calendar_id")
//! //     .summary("团队周会")
//! //     .start_time("2024-07-01T10:00:00")
//! //     .end_time("2024-07-01T11:00:00")
//! //     .build();
//! // let event_result = calendar.v4.calendar_event.create(create_event_req, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v4版本，是最新的稳定版本，提供：
//! - 完整的日历和日程管理功能
//! - 高性能的批量操作支持
//! - 丰富的查询和筛选选项
//! - 企业级的安全和权限控制
//!
//! # 集成特性
//!
//! - 📱 移动端和桌面端同步
//! - 🔗 第三方日历系统集成
//! - 📊 数据分析和报表支持
//! - 🔒 企业安全策略兼容

use crate::core::config::Config;

/// 日历服务 v4 版本
pub mod v4;

use v4::V4;

/// 日历服务
///
/// 飞书日历功能的统一管理入口，提供完整的时间管理和日程协作能力。
/// 支持个人日程管理、团队协作、会议室预订等企业级需求。
///
/// # 服务架构
///
/// - **v4**: 最新版本API，提供完整的日历功能集
///
/// # 核心特性
///
/// - 🚀 高性能日程查询和操作
/// - 👥 多人协作和冲突检测
/// - 🔔 智能提醒和通知系统
/// - 📱 跨平台同步和访问
/// - 🔐 企业级权限和安全控制
///
/// # 适用场景
///
/// - 企业团队日程协调
/// - 会议室资源管理
/// - 项目时间规划
/// - 个人时间管理
/// - 跨部门协作安排
///
/// # 最佳实践
///
/// - 合理设置日程提醒时间
/// - 及时更新参与状态
/// - 避免会议室预订冲突
/// - 定期清理过期日程
/// - 使用重复规则减少重复操作
pub struct CalendarService {
    /// v4版本API服务
    pub v4: V4,
}

impl CalendarService {
    /// 创建新的日历服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的日历服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v4: V4::new(config),
        }
    }

    /// 使用共享配置（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            v4: V4::new(shared.as_ref().clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_calendar_service_creation() {
        let config = Config::default();
        let service = CalendarService::new(config.clone());

        // Verify all 10 sub-services are configured correctly
        assert_eq!(service.v4.calendar.config.app_id, config.app_id);
        assert_eq!(service.v4.calendar.config.app_secret, config.app_secret);
        assert_eq!(service.v4.calendar_acl.config.app_id, config.app_id);
        assert_eq!(service.v4.calendar_event.config.app_id, config.app_id);
        assert_eq!(service.v4.meeting_chat.config.app_secret, config.app_secret);
        assert_eq!(service.v4.meeting_minute.config.app_id, config.app_id);
        assert_eq!(service.v4.timeoff_event.config.app_id, config.app_id);
        assert_eq!(
            service.v4.meeting_room_event.config.app_secret,
            config.app_secret
        );
        assert_eq!(service.v4.attendee.config.app_id, config.app_id);
        assert_eq!(service.v4.setting.config.app_id, config.app_id);
        assert_eq!(
            service.v4.exchange_binding.config.app_secret,
            config.app_secret
        );
    }

    #[test]
    fn test_calendar_service_with_custom_config() {
        let config = Config::builder()
            .app_id("calendar_test_app")
            .app_secret("calendar_test_secret")
            .req_timeout(Duration::from_secs(440))
            .build();

        let service = CalendarService::new(config.clone());

        assert_eq!(service.v4.calendar.config.app_id, "calendar_test_app");
        assert_eq!(
            service.v4.calendar.config.app_secret,
            "calendar_test_secret"
        );
        assert_eq!(
            service.v4.calendar.config.req_timeout,
            Some(Duration::from_secs(440))
        );
        assert_eq!(service.v4.calendar_acl.config.app_id, "calendar_test_app");
        assert_eq!(
            service.v4.calendar_event.config.req_timeout,
            Some(Duration::from_secs(440))
        );
        assert_eq!(service.v4.meeting_chat.config.app_id, "calendar_test_app");
        assert_eq!(
            service.v4.meeting_minute.config.req_timeout,
            Some(Duration::from_secs(440))
        );
        assert_eq!(service.v4.timeoff_event.config.app_id, "calendar_test_app");
        assert_eq!(
            service.v4.meeting_room_event.config.req_timeout,
            Some(Duration::from_secs(440))
        );
        assert_eq!(service.v4.attendee.config.app_id, "calendar_test_app");
        assert_eq!(
            service.v4.setting.config.req_timeout,
            Some(Duration::from_secs(440))
        );
        assert_eq!(
            service.v4.exchange_binding.config.app_id,
            "calendar_test_app"
        );
    }

    #[test]
    fn test_calendar_service_config_independence() {
        let config1 = Config::builder().app_id("calendar_app_1").build();

        let config2 = Config::builder().app_id("calendar_app_2").build();

        let service1 = CalendarService::new(config1);
        let service2 = CalendarService::new(config2);

        assert_eq!(service1.v4.calendar.config.app_id, "calendar_app_1");
        assert_eq!(service2.v4.calendar.config.app_id, "calendar_app_2");
        assert_ne!(
            service1.v4.calendar.config.app_id,
            service2.v4.calendar.config.app_id
        );
        assert_ne!(
            service1.v4.calendar_acl.config.app_id,
            service2.v4.calendar_acl.config.app_id
        );
        assert_ne!(
            service1.v4.calendar_event.config.app_id,
            service2.v4.calendar_event.config.app_id
        );
        assert_ne!(
            service1.v4.meeting_chat.config.app_id,
            service2.v4.meeting_chat.config.app_id
        );
        assert_ne!(
            service1.v4.meeting_minute.config.app_id,
            service2.v4.meeting_minute.config.app_id
        );
        assert_ne!(
            service1.v4.timeoff_event.config.app_id,
            service2.v4.timeoff_event.config.app_id
        );
        assert_ne!(
            service1.v4.meeting_room_event.config.app_id,
            service2.v4.meeting_room_event.config.app_id
        );
        assert_ne!(
            service1.v4.attendee.config.app_id,
            service2.v4.attendee.config.app_id
        );
        assert_ne!(
            service1.v4.setting.config.app_id,
            service2.v4.setting.config.app_id
        );
        assert_ne!(
            service1.v4.exchange_binding.config.app_id,
            service2.v4.exchange_binding.config.app_id
        );
    }

    #[test]
    fn test_calendar_service_sub_services_accessible() {
        let config = Config::default();
        let service = CalendarService::new(config.clone());

        // Test that all sub-services are accessible
        assert_eq!(service.v4.calendar.config.app_id, config.app_id);
        assert_eq!(service.v4.calendar_acl.config.app_id, config.app_id);
        assert_eq!(service.v4.calendar_event.config.app_id, config.app_id);
        assert_eq!(service.v4.meeting_chat.config.app_id, config.app_id);
        assert_eq!(service.v4.meeting_minute.config.app_id, config.app_id);
        assert_eq!(service.v4.timeoff_event.config.app_id, config.app_id);
        assert_eq!(service.v4.meeting_room_event.config.app_id, config.app_id);
        assert_eq!(service.v4.attendee.config.app_id, config.app_id);
        assert_eq!(service.v4.setting.config.app_id, config.app_id);
        assert_eq!(service.v4.exchange_binding.config.app_id, config.app_id);
    }

    #[test]
    fn test_calendar_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = CalendarService::new(config.clone());

        assert_eq!(service.v4.calendar.config.app_id, "clone_test_app");
        assert_eq!(service.v4.calendar.config.app_secret, "clone_test_secret");
        assert_eq!(
            service.v4.calendar_acl.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(service.v4.calendar_event.config.app_id, "clone_test_app");
        assert_eq!(
            service.v4.meeting_chat.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(service.v4.meeting_minute.config.app_id, "clone_test_app");
        assert_eq!(
            service.v4.timeoff_event.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(
            service.v4.meeting_room_event.config.app_id,
            "clone_test_app"
        );
        assert_eq!(service.v4.attendee.config.app_secret, "clone_test_secret");
        assert_eq!(service.v4.setting.config.app_id, "clone_test_app");
        assert_eq!(
            service.v4.exchange_binding.config.app_secret,
            "clone_test_secret"
        );
    }

    #[test]
    fn test_calendar_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(450))
            .build();

        let service = CalendarService::new(config);

        // Verify timeout is propagated to all sub-services
        assert_eq!(
            service.v4.calendar.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.calendar_acl.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.calendar_event.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.meeting_chat.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.meeting_minute.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.timeoff_event.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.meeting_room_event.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.attendee.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.setting.config.req_timeout,
            Some(Duration::from_secs(450))
        );
        assert_eq!(
            service.v4.exchange_binding.config.req_timeout,
            Some(Duration::from_secs(450))
        );
    }

    #[test]
    fn test_calendar_service_multiple_instances() {
        let config = Config::default();

        let service1 = CalendarService::new(config.clone());
        let service2 = CalendarService::new(config.clone());

        // Both services should have the same config values
        assert_eq!(
            service1.v4.calendar.config.app_id,
            service2.v4.calendar.config.app_id
        );
        assert_eq!(
            service1.v4.calendar.config.app_secret,
            service2.v4.calendar.config.app_secret
        );
        assert_eq!(
            service1.v4.calendar_acl.config.app_id,
            service2.v4.calendar_acl.config.app_id
        );
        assert_eq!(
            service1.v4.calendar_event.config.app_secret,
            service2.v4.calendar_event.config.app_secret
        );
        assert_eq!(
            service1.v4.meeting_chat.config.app_id,
            service2.v4.meeting_chat.config.app_id
        );
        assert_eq!(
            service1.v4.meeting_minute.config.app_secret,
            service2.v4.meeting_minute.config.app_secret
        );
        assert_eq!(
            service1.v4.timeoff_event.config.app_id,
            service2.v4.timeoff_event.config.app_id
        );
        assert_eq!(
            service1.v4.meeting_room_event.config.app_secret,
            service2.v4.meeting_room_event.config.app_secret
        );
        assert_eq!(
            service1.v4.attendee.config.app_id,
            service2.v4.attendee.config.app_id
        );
        assert_eq!(
            service1.v4.setting.config.app_secret,
            service2.v4.setting.config.app_secret
        );
        assert_eq!(
            service1.v4.exchange_binding.config.app_id,
            service2.v4.exchange_binding.config.app_id
        );
    }

    #[test]
    fn test_calendar_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(460))
            .build();

        let service = CalendarService::new(config);

        // Verify all sub-services have consistent configurations
        let configs = [
            &service.v4.calendar.config,
            &service.v4.calendar_acl.config,
            &service.v4.calendar_event.config,
            &service.v4.meeting_chat.config,
            &service.v4.meeting_minute.config,
            &service.v4.timeoff_event.config,
            &service.v4.meeting_room_event.config,
            &service.v4.attendee.config,
            &service.v4.setting.config,
            &service.v4.exchange_binding.config,
        ];

        for config in &configs {
            assert_eq!(config.app_id, "consistency_test");
            assert_eq!(config.app_secret, "consistency_secret");
            assert_eq!(config.req_timeout, Some(Duration::from_secs(460)));
        }
    }
}
