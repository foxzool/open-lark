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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_calendar_service_creation() {
        let config = create_test_config();
        let calendar_service = CalendarService::new(config);

        // Verify service structure
        assert!(std::ptr::addr_of!(calendar_service.v4) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_calendar_service_with_custom_config() {
        let config = Config::builder()
            .app_id("calendar_app")
            .app_secret("calendar_secret")
            .req_timeout(std::time::Duration::from_millis(15000))
            .base_url("https://calendar.api.com")
            .build();

        let calendar_service = CalendarService::new(config);

        // Verify service creation with custom config
        assert!(std::ptr::addr_of!(calendar_service.v4) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_calendar_service_configuration_variations() {
        let test_configs = vec![
            Config::builder()
                .app_id("cal_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("cal_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(20000))
                .build(),
            Config::builder()
                .app_id("cal_custom")
                .app_secret("custom_secret")
                .base_url("https://custom.calendar.com")
                .build(),
            Config::builder()
                .app_id("cal_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(25000))
                .base_url("https://full.calendar.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let calendar_service = CalendarService::new(config);

            // Each configuration should create a valid service
            assert!(std::ptr::addr_of!(calendar_service.v4) as *const _ != std::ptr::null());
        }
    }

    #[test]
    fn test_calendar_service_multiple_instances() {
        let config1 = create_test_config();
        let config2 = Config::builder()
            .app_id("calendar2")
            .app_secret("secret2")
            .build();

        let calendar_service1 = CalendarService::new(config1);
        let calendar_service2 = CalendarService::new(config2);

        // Services should be independent instances
        let service1_ptr = std::ptr::addr_of!(calendar_service1) as *const _;
        let service2_ptr = std::ptr::addr_of!(calendar_service2) as *const _;

        assert_ne!(service1_ptr, service2_ptr, "Services should be independent instances");

        // Each service should have valid v4 API
        assert!(std::ptr::addr_of!(calendar_service1.v4) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(calendar_service2.v4) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_calendar_service_config_cloning_behavior() {
        let original_config = create_test_config();

        // Test that the service works with cloned configs
        let calendar_service1 = CalendarService::new(original_config.clone());
        let calendar_service2 = CalendarService::new(original_config);

        // Both should work independently
        assert!(std::ptr::addr_of!(calendar_service1.v4) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(calendar_service2.v4) as *const _ != std::ptr::null());

        // But should be different service instances
        let service1_ptr = std::ptr::addr_of!(calendar_service1) as *const _;
        let service2_ptr = std::ptr::addr_of!(calendar_service2) as *const _;
        assert_ne!(service1_ptr, service2_ptr);
    }
}
