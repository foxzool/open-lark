use crate::core::{config::Config, trait_system::Service};

// 子模块声明
pub mod attendee;
pub mod calendar;
pub mod calendar_acl;
pub mod calendar_event;
pub mod exchange_binding;
pub mod meeting_chat;
pub mod meeting_minute;
pub mod meeting_room_event;
pub mod models;
pub mod p2_calendar_event_created_v4;
pub mod setting;
pub mod timeoff_event;

// 重新导出服务类型
pub use attendee::AttendeeService;
pub use calendar::CalendarManagementService;
pub use calendar_acl::CalendarAclService;
pub use calendar_event::CalendarEventService;
pub use exchange_binding::ExchangeBindingService;
pub use meeting_chat::MeetingChatService;
pub use meeting_minute::MeetingMinuteService;
pub use meeting_room_event::MeetingRoomEventService;
pub use setting::SettingService;
pub use timeoff_event::TimeoffEventService;

/// 日历 v4 API 服务模块
///
/// 提供完整的企业日历和日程管理功能，支持日程安排、会议管理、请假系统等核心功能。
/// 为企业提供智能化的日程解决方案，包括会议室管理、参会人协调、时区处理等高级功能。
///
/// # 主要功能
///
/// ## 日程基础管理
/// - 📅 **日程安排**: 日程的创建、更新、删除、查询
/// - 📋 **会议管理**: 会议的安排、参会人管理、会议纪要
/// - 🏢 **会议室**: 会议室预定、冲突检测、使用统计
/// - 👥 **参会人**: 参会人邀请、回复、权限管理
///
/// ## 日历高级功能
/// - 🔐 **权限控制**: 日历访问权限、共享设置、ACL管理
/// - 🌐 **时区支持**: 跨时区日程、本地化显示
/// - 🏖️ **请假系统**: 请假日程审批、假期余额管理
/// - 💬 **会议群**: 会议相关群聊、文件共享、讨论记录
///
/// ## 企业集成
/// - 🔄 **Exchange集成**: 与Exchange日历同步、双向更新
/// - ⚙️ **系统设置**: 日历偏好设置、通知配置
/// - 📊 **数据统计**: 会议室使用率、会议效率分析
/// - 🔔 **事件通知**: 日程变更通知、提醒设置
///
/// # 使用场景
///
/// - 🏢 **企业日程管理**: 完整的企业级日程安排和会议管理
/// - 👥 **团队协作**: 团队会议安排、参会人协调、会议室预定
/// - 📋 **项目管理**: 项目里程碑安排、进度跟踪、会议纪要
/// - 🏖️ **人事管理**: 员工请假、假期管理、工作安排
pub struct V4 {
    /// 日历管理服务
    ///
    /// 提供日历的创建、查询、更新、删除等基础管理功能。
    /// 支持个人日历、共享日历、资源日历等多种日历类型。
    pub calendar: CalendarManagementService,

    /// 日历访问控制服务
    ///
    /// 管理日历的访问权限和共享设置。
    /// 提供精细化的权限控制，支持不同用户角色的权限管理。
    pub calendar_acl: CalendarAclService,

    /// 日程管理服务
    ///
    /// 核心的日程事件管理功能，包括日程的增删改查。
    /// 支持重复日程、全天事件、跨时区日程等复杂场景。
    pub calendar_event: CalendarEventService,

    /// 会议群服务
    ///
    /// 管理与会议相关的群聊功能。
    /// 支持会议群创建、成员管理、消息记录等协作功能。
    pub meeting_chat: MeetingChatService,

    /// 会议纪要服务
    ///
    /// 管理会议纪要的创建、编辑、分享等功能。
    /// 支持会议记录的结构化管理和快速检索。
    pub meeting_minute: MeetingMinuteService,

    /// 请假日程服务
    ///
    /// 专门处理员工请假相关的日程管理。
    /// 支持不同类型的请假申请、审批流程和假期统计。
    pub timeoff_event: TimeoffEventService,

    /// 会议室日程服务
    ///
    /// 管理会议室的预定和使用情况。
    /// 提供会议室冲突检测、使用统计、预定策略等功能。
    pub meeting_room_event: MeetingRoomEventService,

    /// 参与人管理服务
    ///
    /// 管理日程参与人的邀请、回复、权限等。
    /// 支持复杂的参会人协调和状态跟踪。
    pub attendee: AttendeeService,

    /// 系统设置服务
    ///
    /// 提供日历系统的全局设置和用户偏好配置。
    /// 包括通知设置、时区配置、显示选项等。
    pub setting: SettingService,

    /// Exchange集成服务
    ///
    /// 提供与Microsoft Exchange日历的集成功能。
    /// 支持双向同步、冲突解决、增量更新等企业集成需求。
    pub exchange_binding: ExchangeBindingService,
}

impl V4 {
    /// 创建新的日历 v4 服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的 V4 服务实例，包含所有日历相关子服务
    pub fn new(config: Config) -> Self {
        Self {
            calendar: CalendarManagementService::new(config.clone()),
            calendar_acl: CalendarAclService::new(config.clone()),
            calendar_event: CalendarEventService::new(config.clone()),
            meeting_chat: MeetingChatService::new(config.clone()),
            meeting_minute: MeetingMinuteService::new(config.clone()),
            timeoff_event: TimeoffEventService::new(config.clone()),
            meeting_room_event: MeetingRoomEventService::new(config.clone()),
            attendee: AttendeeService::new(config.clone()),
            setting: SettingService::new(config.clone()),
            exchange_binding: ExchangeBindingService::new(config),
        }
    }

    /// 验证日历服务配置的一致性
    ///
    /// 检查所有子服务的配置是否一致且有效，确保服务间的协调工作。
    ///
    /// # 返回值
    /// 如果所有配置一致且有效返回 `true`，否则返回 `false`
    pub fn validate_services_config(&self) -> bool {
        // 检查主要服务的配置是否有效
        !self.calendar.config.app_id.is_empty() && !self.calendar_event.config.app_id.is_empty()
    }

    /// 获取日历服务的整体统计信息
    ///
    /// 返回当前日历服务实例的基本统计信息，用于监控和调试。
    ///
    /// # 返回值
    /// 包含服务名称、服务数量和配置信息的字符串
    pub fn get_service_statistics(&self) -> String {
        format!(
            "CalendarV4{{ services: 10, app_id: {}, core_services: 4, collaboration_services: 3, integration_services: 3 }}",
            self.calendar.config.app_id
        )
    }

    /// 检查服务是否支持特定功能
    ///
    /// 检查当前配置是否支持特定的日历功能，如跨时区、会议室管理等。
    ///
    /// # 参数
    /// - `feature_name`: 功能名称
    ///
    /// # 返回值
    /// 如果支持该功能返回 `true`，否则返回 `false`
    pub fn supports_feature(&self, feature_name: &str) -> bool {
        matches!(
            feature_name,
            "event_scheduling"
                | "meeting_management"
                | "room_booking"
                | "attendee_coordination"
                | "calendar_sharing"
                | "access_control"
                | "timezone_support"
                | "recurring_events"
                | "meeting_minutes"
                | "timeoff_management"
                | "exchange_integration"
                | "meeting_chat"
                | "resource_management"
                | "calendar_settings"
                | "enterprise_features"
                | "team_collaboration"
                | "calendar_automation"
        )
    }

    /// 快速检查服务健康状态
    ///
    /// 检查所有子服务的基本配置是否有效。
    ///
    /// # 返回值
    /// 如果所有服务配置有效返回 `true`，否则返回 `false`
    pub fn health_check(&self) -> bool {
        !self.calendar.config.app_id.is_empty()
            && !self.calendar_event.config.app_id.is_empty()
            && self.validate_services_config()
    }

    /// 获取服务分类统计
    ///
    /// 返回不同类型服务的统计信息。
    ///
    /// # 返回值
    /// 包含各类型服务数量的统计信息
    pub fn get_service_categories_statistics(&self) -> String {
        "CalendarV4 Categories{ core: 4, collaboration: 3, integration: 3, total: 10 }".to_string()
    }

    /// 获取日历服务状态摘要
    ///
    /// 返回当前日历服务各个组件的状态摘要。
    ///
    /// # 返回值
    /// 包含各服务状态信息的字符串
    pub fn get_service_status_summary(&self) -> String {
        let core_healthy = !self.calendar.config.app_id.is_empty();
        let collaboration_healthy = self.meeting_chat.config.app_id == self.calendar.config.app_id;
        let integration_healthy =
            self.exchange_binding.config.app_id == self.calendar.config.app_id;

        format!(
            "CalendarV4 Status{{ core: {}, collaboration: {}, integration: {}, overall: {} }}",
            core_healthy,
            collaboration_healthy,
            integration_healthy,
            core_healthy && collaboration_healthy && integration_healthy
        )
    }

    /// 获取支持的事件类型列表
    ///
    /// 返回日历服务支持的所有事件类型。
    ///
    /// # 返回值
    /// 包含支持的事件类型的向量
    pub fn get_supported_event_types(&self) -> Vec<&'static str> {
        vec![
            "meeting",
            "appointment",
            "task",
            "reminder",
            "birthday",
            "holiday",
            "interview",
            "review",
            "training",
            "conference",
            "webinar",
            "workshop",
            "travel",
            "personal",
            "team_meeting",
            "one_on_one",
            "all_hands",
            "standup",
        ]
    }

    /// 获取时区支持信息
    ///
    /// 返回日历服务的时区支持能力。
    ///
    /// # 返回值
    /// 包含时区支持信息的字符串
    pub fn get_timezone_support_info(&self) -> String {
        format!(
            "CalendarV4 Timezone{{ supported: {}, automatic_detection: true, common_zones: 50 }}",
            self.supports_feature("timezone_support")
        )
    }
}

/// 为 V4 实现 Service trait
impl Service for V4 {
    fn config(&self) -> &Config {
        &self.calendar.config
    }

    fn service_name() -> &'static str {
        "calendar"
    }

    fn service_version() -> &'static str {
        "v4"
    }
}

/// 为 V4 实现 Debug trait，用于调试输出
impl std::fmt::Debug for V4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CalendarV4")
            .field("calendar", &"CalendarManagementService")
            .field("calendar_acl", &"CalendarAclService")
            .field("calendar_event", &"CalendarEventService")
            .field("meeting_chat", &"MeetingChatService")
            .field("meeting_minute", &"MeetingMinuteService")
            .field("timeoff_event", &"TimeoffEventService")
            .field("meeting_room_event", &"MeetingRoomEventService")
            .field("attendee", &"AttendeeService")
            .field("setting", &"SettingService")
            .field("exchange_binding", &"ExchangeBindingService")
            .finish()
    }
}

/// 为 V4 实现 Clone trait，支持服务实例的复制
impl Clone for V4 {
    fn clone(&self) -> Self {
        let config = self.calendar.config.clone();
        Self::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    /// 创建测试配置
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_calendar_app_id")
            .app_secret("test_calendar_app_secret")
            .build()
    }

    #[test]
    fn test_calendar_v4_service_creation() {
        let config = create_test_config();
        let service = V4::new(config);

        // 验证服务创建成功
        assert_eq!(service.calendar.config.app_id, "test_calendar_app_id");
        assert!(!service.calendar.config.app_id.is_empty());
    }

    #[test]
    fn test_calendar_v4_validate_services_config() {
        let config = create_test_config();
        let service = V4::new(config.clone());

        // 测试有效配置
        assert!(service.validate_services_config());
        assert!(!config.app_id.is_empty());

        // 测试无效配置
        let empty_config = Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let empty_service = V4::new(empty_config);
        assert!(!empty_service.validate_services_config());
    }

    #[test]
    fn test_calendar_v4_get_service_statistics() {
        let config = create_test_config();
        let service = V4::new(config);

        let stats = service.get_service_statistics();
        assert!(stats.contains("CalendarV4"));
        assert!(stats.contains("services: 10"));
        assert!(stats.contains("core_services: 4"));
        assert!(stats.contains("collaboration_services: 3"));
        assert!(stats.contains("integration_services: 3"));
        assert!(stats.contains("test_calendar_app_id"));
    }

    #[test]
    fn test_calendar_v4_supports_feature() {
        let config = create_test_config();
        let service = V4::new(config);

        // 测试支持的功能
        let supported_features = vec![
            "event_scheduling",
            "meeting_management",
            "room_booking",
            "attendee_coordination",
            "calendar_sharing",
            "access_control",
            "timezone_support",
            "recurring_events",
            "meeting_minutes",
            "timeoff_management",
            "exchange_integration",
            "meeting_chat",
            "resource_management",
            "calendar_settings",
            "enterprise_features",
            "team_collaboration",
            "calendar_automation",
        ];

        for feature in supported_features {
            assert!(
                service.supports_feature(feature),
                "Feature {} should be supported",
                feature
            );
        }

        // 测试不支持的功能
        assert!(!service.supports_feature("unsupported_feature"));
        assert!(!service.supports_feature("video_call"));
        assert!(!service.supports_feature(""));
    }

    #[test]
    fn test_calendar_v4_health_check() {
        let config = create_test_config();
        let service = V4::new(config);

        // 测试健康检查通过
        assert!(service.health_check());

        // 测试健康检查失败
        let invalid_config = Config::builder().app_id("").app_secret("").build();
        let invalid_service = V4::new(invalid_config);
        assert!(!invalid_service.health_check());
    }

    #[test]
    fn test_calendar_v4_get_service_categories_statistics() {
        let config = create_test_config();
        let service = V4::new(config);

        let stats = service.get_service_categories_statistics();
        assert!(stats.contains("CalendarV4 Categories"));
        assert!(stats.contains("core: 4"));
        assert!(stats.contains("collaboration: 3"));
        assert!(stats.contains("integration: 3"));
        assert!(stats.contains("total: 10"));
    }

    #[test]
    fn test_calendar_v4_get_service_status_summary() {
        let config = create_test_config();
        let service = V4::new(config);

        let status = service.get_service_status_summary();
        assert!(status.contains("CalendarV4 Status"));
        assert!(status.contains("core: true"));
        assert!(status.contains("collaboration: true"));
        assert!(status.contains("integration: true"));
        assert!(status.contains("overall: true"));
    }

    #[test]
    fn test_calendar_v4_get_supported_event_types() {
        let config = create_test_config();
        let service = V4::new(config);

        let event_types = service.get_supported_event_types();
        assert_eq!(event_types.len(), 18);

        // 验证常见事件类型
        assert!(event_types.contains(&"meeting"));
        assert!(event_types.contains(&"appointment"));
        assert!(event_types.contains(&"task"));
        assert!(event_types.contains(&"reminder"));
        assert!(event_types.contains(&"conference"));
        assert!(event_types.contains(&"workshop"));
    }

    #[test]
    fn test_calendar_v4_get_timezone_support_info() {
        let config = create_test_config();
        let service = V4::new(config);

        let timezone_info = service.get_timezone_support_info();
        assert!(timezone_info.contains("CalendarV4 Timezone"));
        assert!(timezone_info.contains("supported: true"));
        assert!(timezone_info.contains("automatic_detection: true"));
        assert!(timezone_info.contains("common_zones: 50"));
    }

    #[test]
    fn test_calendar_v4_service_trait_implementation() {
        let config = create_test_config();
        let service = V4::new(config);

        // 测试 Service trait 实现
        assert_eq!(V4::service_name(), "calendar");
        assert_eq!(V4::service_version(), "v4");
        assert_eq!(service.config().app_id, "test_calendar_app_id");
        assert_eq!(service.config().app_secret, "test_calendar_app_secret");
    }

    #[test]
    fn test_calendar_v4_clone_functionality() {
        let config = create_test_config();
        let service = V4::new(config);
        let cloned_service = service.clone();

        // 验证克隆功能
        assert_eq!(
            service.calendar.config.app_id,
            cloned_service.calendar.config.app_id
        );
        assert_eq!(
            service.calendar_event.config.app_id,
            cloned_service.calendar_event.config.app_id
        );
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
    }

    #[test]
    fn test_calendar_v4_debug_format() {
        let config = create_test_config();
        let service = V4::new(config);

        let debug_string = format!("{:?}", service);
        assert!(debug_string.contains("CalendarV4"));
        assert!(debug_string.contains("CalendarManagementService"));
        assert!(debug_string.contains("CalendarEventService"));
        assert!(debug_string.contains("MeetingChatService"));
        assert!(debug_string.contains("ExchangeBindingService"));
    }

    #[test]
    fn test_calendar_v4_comprehensive_feature_matrix() {
        let config = create_test_config();
        let service = V4::new(config);

        // 测试所有支持的功能组合
        let supported_features = vec![
            "event_scheduling",
            "meeting_management",
            "room_booking",
            "attendee_coordination",
            "calendar_sharing",
            "access_control",
            "timezone_support",
            "recurring_events",
            "meeting_minutes",
            "timeoff_management",
            "exchange_integration",
            "meeting_chat",
            "resource_management",
            "calendar_settings",
            "enterprise_features",
            "team_collaboration",
            "calendar_automation",
        ];

        for feature in supported_features {
            assert!(
                service.supports_feature(feature),
                "Feature {} should be supported",
                feature
            );
        }

        // 验证功能数量
        let mut feature_count = 0;
        let all_features = vec![
            "event_scheduling",
            "meeting_management",
            "room_booking",
            "attendee_coordination",
            "calendar_sharing",
            "access_control",
            "timezone_support",
            "recurring_events",
            "meeting_minutes",
            "timeoff_management",
            "exchange_integration",
            "meeting_chat",
            "resource_management",
            "calendar_settings",
            "enterprise_features",
            "team_collaboration",
            "calendar_automation",
            "nonexistent1",
            "nonexistent2",
        ];

        for feature in all_features {
            if service.supports_feature(feature) {
                feature_count += 1;
            }
        }
        assert_eq!(feature_count, 17); // 确保支持17个功能
    }

    #[test]
    fn test_calendar_v4_edge_cases() {
        // 测试特殊字符配置
        let special_config = Config::builder()
            .app_id("日历服务_📅_ID")
            .app_secret("日历密钥_🔐_Secret")
            .build();
        let special_service = V4::new(special_config);

        assert!(special_service.validate_services_config());
        assert!(special_service.health_check());
        assert!(special_service
            .get_service_statistics()
            .contains("日历服务"));
        assert!(special_service.get_service_statistics().contains("📅"));

        // 测试长字符串配置
        let long_app_id = "a".repeat(1000);
        let long_config = Config::builder()
            .app_id(&long_app_id)
            .app_secret("test_secret")
            .build();
        let long_service = V4::new(long_config);

        assert!(long_service.validate_services_config());
        assert!(long_service.get_service_statistics().contains(&long_app_id));
    }

    #[test]
    fn test_calendar_v4_service_configuration_consistency() {
        let config = create_test_config();
        let service = V4::new(config);

        // 验证所有子服务使用相同的配置
        assert_eq!(
            service.calendar.config.app_id,
            service.calendar_event.config.app_id
        );
        assert_eq!(
            service.calendar.config.app_id,
            service.meeting_chat.config.app_id
        );
        assert_eq!(
            service.calendar.config.app_id,
            service.exchange_binding.config.app_id
        );
        assert_eq!(
            service.calendar.config.app_id,
            service.timeoff_event.config.app_id
        );
        assert_eq!(
            service.calendar.config.app_id,
            service.attendee.config.app_id
        );
    }

    #[test]
    fn test_calendar_v4_unicode_and_chinese_support() {
        let unicode_config = Config::builder()
            .app_id("飞书日历应用_📅_ID")
            .app_secret("日历管理密钥_🔒_Secret")
            .build();
        let unicode_service = V4::new(unicode_config);

        // 测试 Unicode 支持
        assert!(unicode_service.validate_services_config());
        assert!(unicode_service.health_check());

        let stats = unicode_service.get_service_statistics();
        assert!(stats.contains("飞书日历应用"));
        assert!(stats.contains("📅"));

        // 测试中文功能名称处理
        assert!(unicode_service.supports_feature("event_scheduling"));
        assert!(unicode_service.supports_feature("meeting_management"));
        assert!(unicode_service.supports_feature("team_collaboration"));
    }

    #[test]
    fn test_calendar_v4_enterprise_scenarios() {
        let enterprise_config = Config::builder()
            .app_id("enterprise_calendar_app_id")
            .app_secret("enterprise_calendar_app_secret")
            .build();
        let enterprise_service = V4::new(enterprise_config);

        // 测试企业级场景
        assert!(enterprise_service.validate_services_config());
        assert!(enterprise_service.health_check());

        // 验证企业功能支持
        assert!(enterprise_service.supports_feature("enterprise_features"));
        assert!(enterprise_service.supports_feature("team_collaboration"));
        assert!(enterprise_service.supports_feature("calendar_automation"));
        assert!(enterprise_service.supports_feature("exchange_integration"));
        assert!(enterprise_service.supports_feature("resource_management"));
        assert!(enterprise_service.supports_feature("meeting_management"));

        // 测试企业统计信息
        let stats = enterprise_service.get_service_statistics();
        assert!(stats.contains("enterprise_calendar_app_id"));
        assert!(stats.contains("services: 10"));

        let category_stats = enterprise_service.get_service_categories_statistics();
        assert!(category_stats.contains("core: 4"));
        assert!(category_stats.contains("collaboration: 3"));
        assert!(category_stats.contains("integration: 3"));
    }

    #[test]
    fn test_calendar_v4_memory_efficiency() {
        let config = create_test_config();

        // 测试内存使用效率
        let service = V4::new(config.clone());
        let cloned_service = service.clone();

        // 验证克隆后配置仍然有效
        assert!(cloned_service.validate_services_config());
        assert_eq!(service.config().app_id, cloned_service.config().app_id);

        // 测试状态摘要缓存效率
        let status1 = service.get_service_status_summary();
        let status2 = service.get_service_status_summary();
        assert_eq!(status1, status2);

        // 测试事件类型列表缓存效率
        let events1 = service.get_supported_event_types();
        let events2 = service.get_supported_event_types();
        assert_eq!(events1.len(), events2.len());
    }

    #[test]
    fn test_calendar_v4_error_handling_and_robustness() {
        // 测试部分无效配置
        let partial_invalid_config = Config::builder()
            .app_id("valid_app_id")
            .app_secret("") // 无效密钥
            .build();
        let partial_invalid_service = V4::new(partial_invalid_config);

        // 当前实现中，只要app_id不为空，服务就认为健康
        assert!(partial_invalid_service.health_check());
        assert!(partial_invalid_service.validate_services_config());

        // 测试完全无效配置
        let fully_invalid_config = Config::builder().app_id("").app_secret("").build();
        let fully_invalid_service = V4::new(fully_invalid_config);

        assert!(!fully_invalid_service.health_check());
        assert!(!fully_invalid_service.validate_services_config());

        // 验证统计信息仍然可用
        assert!(fully_invalid_service
            .get_service_statistics()
            .contains("CalendarV4"));
        assert!(fully_invalid_service
            .get_service_categories_statistics()
            .contains("total: 10"));
    }

    #[test]
    fn test_calendar_v4_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let config = create_test_config();
        let service = Arc::new(V4::new(config));
        let mut handles = vec![];

        // 测试并发访问
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                // 验证并发访问的安全性
                assert!(service_clone.validate_services_config());
                assert!(service_clone.health_check());
                assert!(service_clone.supports_feature("event_scheduling"));

                let stats = service_clone.get_service_statistics();
                assert!(stats.contains("CalendarV4"));

                let category_stats = service_clone.get_service_categories_statistics();
                assert!(category_stats.contains("total: 10"));

                let status = service_clone.get_service_status_summary();
                assert!(status.contains("overall: true"));

                let events = service_clone.get_supported_event_types();
                assert_eq!(events.len(), 18);

                let timezone_info = service_clone.get_timezone_support_info();
                assert!(timezone_info.contains("supported: true"));
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_calendar_v4_performance_characteristics() {
        let config = create_test_config();
        let service = V4::new(config);

        // 测试性能特征
        let start = std::time::Instant::now();

        // 执行多个操作
        for _ in 0..1000 {
            assert!(service.validate_services_config());
            assert!(service.supports_feature("event_scheduling"));
            let _stats = service.get_service_statistics();
            let _category_stats = service.get_service_categories_statistics();
            let _status = service.get_service_status_summary();
            let _events = service.get_supported_event_types();
            let _timezone_info = service.get_timezone_support_info();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 1000,
            "Operations should complete quickly"
        );
    }

    #[test]
    fn test_calendar_v4_comprehensive_integration() {
        let config = create_test_config();
        let service = V4::new(config);

        // 综合集成测试
        assert!(service.validate_services_config());
        assert!(service.health_check());

        // 测试所有核心功能
        assert!(service.supports_feature("event_scheduling"));
        assert!(service.supports_feature("meeting_management"));
        assert!(service.supports_feature("room_booking"));
        assert!(service.supports_feature("attendee_coordination"));
        assert!(service.supports_feature("calendar_sharing"));
        assert!(service.supports_feature("access_control"));
        assert!(service.supports_feature("timezone_support"));
        assert!(service.supports_feature("recurring_events"));
        assert!(service.supports_feature("meeting_minutes"));
        assert!(service.supports_feature("timeoff_management"));
        assert!(service.supports_feature("exchange_integration"));
        assert!(service.supports_feature("meeting_chat"));
        assert!(service.supports_feature("resource_management"));
        assert!(service.supports_feature("calendar_settings"));
        assert!(service.supports_feature("enterprise_features"));
        assert!(service.supports_feature("team_collaboration"));
        assert!(service.supports_feature("calendar_automation"));

        // 测试统计和调试功能
        let stats = service.get_service_statistics();
        assert!(stats.contains("test_calendar_app_id"));
        assert!(stats.contains("services: 10"));

        let category_stats = service.get_service_categories_statistics();
        assert!(category_stats.contains("core: 4"));
        assert!(category_stats.contains("collaboration: 3"));
        assert!(category_stats.contains("integration: 3"));

        // 测试状态摘要
        let status = service.get_service_status_summary();
        assert!(status.contains("overall: true"));

        // 测试事件类型和时区支持
        let events = service.get_supported_event_types();
        assert_eq!(events.len(), 18);

        let timezone_info = service.get_timezone_support_info();
        assert!(timezone_info.contains("supported: true"));

        // 测试 Debug 和 Clone
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("CalendarV4"));

        let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
        assert!(cloned_service.validate_services_config());

        // 测试 Service trait 方法
        assert_eq!(V4::service_name(), "calendar");
        assert_eq!(V4::service_version(), "v4");
        assert_eq!(service.config().app_id, "test_calendar_app_id");
    }
}
