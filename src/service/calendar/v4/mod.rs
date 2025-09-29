use crate::core::config::Config;

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

/// 日历服务 v4 版本聚合器
///
/// 提供所有日历 v4 API 的访问接口
pub struct V4 {
    /// 日历管理服务
    pub calendar: CalendarManagementService,
    /// 日历访问控制服务
    pub calendar_acl: CalendarAclService,
    /// 日程管理服务
    pub calendar_event: CalendarEventService,
    /// 会议群服务
    pub meeting_chat: MeetingChatService,
    /// 会议纪要服务
    pub meeting_minute: MeetingMinuteService,
    /// 请假日程服务
    pub timeoff_event: TimeoffEventService,
    /// 会议室日程服务
    pub meeting_room_event: MeetingRoomEventService,
    /// 参与人管理服务
    pub attendee: AttendeeService,
    /// 设置服务
    pub setting: SettingService,
    /// Exchange绑定服务
    pub exchange_binding: ExchangeBindingService,
}

impl V4 {
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
}
