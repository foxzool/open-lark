//! API端点定义
//!
//! 提供类型安全的API端点管理，替代字符串拼接方式。

// ==================== Calendar（日历）v4 ====================

/// Calendar API V4 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CalendarApiV4 {
    // ========== Calendar 资源 ==========
    /// 获取日历信息
    /// GET /open-apis/calendar/v4/calendars/:calendar_id
    CalendarGet(String),

    /// 创建日历
    /// POST /open-apis/calendar/v4/calendars
    CalendarCreate,

    /// 删除日历
    /// DELETE /open-apis/calendar/v4/calendars/:calendar_id
    CalendarDelete(String),

    /// 更新日历
    /// PATCH /open-apis/calendar/v4/calendars/:calendar_id
    CalendarPatch(String),

    /// 获取日历列表
    /// GET /open-apis/calendar/v4/calendars
    CalendarList,

    /// 批量获取日历信息
    /// POST /open-apis/calendar/v4/calendars/batch_get
    CalendarBatchGet,

    /// 搜索日历
    /// GET /open-apis/calendar/v4/calendars/search
    CalendarSearch,

    /// 设置主日历
    /// PUT /open-apis/calendar/v4/calendars/primary
    CalendarPrimary,

    /// 获取主日历
    /// GET /open-apis/calendar/v4/calendars/primary
    CalendarPrimaryGet,

    // ========== Calendar ACL 权限管理 ==========
    /// 创建日历 ACL
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/acls
    CalendarAclCreate(String),

    /// 删除日历 ACL
    /// DELETE /open-apis/calendar/v4/calendars/:calendar_id/acls/:acl_type/:rule_id
    CalendarAclDelete(String, String, String),

    /// 获取日历 ACL 列表
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/acls
    CalendarAclList(String),

    // ========== Event 日程管理 ==========
    /// 获取日程信息
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id
    EventGet(String, String),

    /// 创建日程
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events
    EventCreate(String),

    /// 删除日程
    /// DELETE /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id
    EventDelete(String, String),

    /// 更新日程
    /// PATCH /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id
    EventPatch(String, String),

    /// 获取日程列表
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/events
    EventList(String),

    /// 批量获取日程信息
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events/batch_get
    EventBatchGet(String),

    /// 回复日程
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/chat_member
    EventReply(String, String),

    // ========== Event Attendee 参会人管理 ==========
    /// 创建参会人
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees
    EventAttendeeCreate(String, String),

    /// 删除参会人
    /// DELETE /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/:attendee_id
    EventAttendeeDelete(String, String, String),

    /// 获取参会人列表
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees
    EventAttendeeList(String, String),

    /// 批量删除参会人
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/batch_delete
    EventAttendeeBatchDelete(String, String),

    /// 获取会议群成员
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/chat_member
    EventAttendeeChatMemberList(String, String),

    // ========== Freebusy 忙闲查询 ==========
    /// 查询忙闲信息
    /// POST /open-apis/calendar/v4/freebusy/query
    FreebusyQuery,

    // ========== Setting 设置 ==========
    /// 获取日历设置
    /// GET /open-apis/calendar/v4/settings
    SettingGet,

    /// 更新日历设置
    /// PUT /open-apis/calendar/v4/settings
    SettingUpdate,

    // ========== Timeoff Event 请假日程 ==========
    /// 创建请假日程
    /// POST /open-apis/calendar/v4/timeoff_events
    TimeoffEventCreate,

    /// 获取请假日程列表
    /// GET /open-apis/calendar/v4/timeoff_events
    TimeoffEventList,

    /// 删除请假日程
    /// DELETE /open-apis/calendar/v4/timeoff_events/:event_id
    TimeoffEventDelete(String),

    // ========== Exchange Binding ==========
    /// 获取 Exchange 绑定
    /// GET /open-apis/calendar/v4/exchange_bindings
    ExchangeBindingGet,

    /// 创建 Exchange 绑定
    /// POST /open-apis/calendar/v4/exchange_bindings
    ExchangeBindingCreate,

    /// 删除 Exchange 绑定
    /// DELETE /open-apis/calendar/v4/exchange_bindings/:exchange_id
    ExchangeBindingDelete(String),

    // ========== Meeting Chat ==========
    /// 获取会议群
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat
    MeetingChatGet(String, String),

    // ========== Meeting Minute ==========
    /// 获取会议纪要
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_minute
    MeetingMinuteGet(String, String),

    /// 更新会议纪要
    /// PATCH /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_minute
    MeetingMinutePatch(String, String),

    // ========== Subscription 订阅 ==========
    /// 订阅日历
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/subscribe
    CalendarSubscribe(String),

    /// 取消订阅日历
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/unsubscribe
    CalendarUnsubscribe(String),
}

impl CalendarApiV4 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // Calendar 资源
            CalendarApiV4::CalendarGet(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{}", calendar_id)
            }
            CalendarApiV4::CalendarCreate => "/open-apis/calendar/v4/calendars".to_string(),
            CalendarApiV4::CalendarDelete(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{}", calendar_id)
            }
            CalendarApiV4::CalendarPatch(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{}", calendar_id)
            }
            CalendarApiV4::CalendarList => "/open-apis/calendar/v4/calendars".to_string(),
            CalendarApiV4::CalendarBatchGet => {
                "/open-apis/calendar/v4/calendars/batch_get".to_string()
            }
            CalendarApiV4::CalendarSearch => "/open-apis/calendar/v4/calendars/search".to_string(),
            CalendarApiV4::CalendarPrimary => {
                "/open-apis/calendar/v4/calendars/primary".to_string()
            }
            CalendarApiV4::CalendarPrimaryGet => {
                "/open-apis/calendar/v4/calendars/primary".to_string()
            }

            // Calendar ACL
            CalendarApiV4::CalendarAclCreate(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{}/acls", calendar_id)
            }
            CalendarApiV4::CalendarAclDelete(calendar_id, acl_type, rule_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/acls/{}/{}",
                    calendar_id, acl_type, rule_id
                )
            }
            CalendarApiV4::CalendarAclList(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{}/acls", calendar_id)
            }

            // Event
            CalendarApiV4::EventGet(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}",
                    calendar_id, event_id
                )
            }
            CalendarApiV4::EventCreate(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{}/events", calendar_id)
            }
            CalendarApiV4::EventDelete(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}",
                    calendar_id, event_id
                )
            }
            CalendarApiV4::EventPatch(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}",
                    calendar_id, event_id
                )
            }
            CalendarApiV4::EventList(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{}/events", calendar_id)
            }
            CalendarApiV4::EventBatchGet(calendar_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/batch_get",
                    calendar_id
                )
            }
            CalendarApiV4::EventReply(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/attendees/chat_member",
                    calendar_id, event_id
                )
            }

            // Event Attendee
            CalendarApiV4::EventAttendeeCreate(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/attendees",
                    calendar_id, event_id
                )
            }
            CalendarApiV4::EventAttendeeDelete(calendar_id, event_id, attendee_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/attendees/{}",
                    calendar_id, event_id, attendee_id
                )
            }
            CalendarApiV4::EventAttendeeList(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/attendees",
                    calendar_id, event_id
                )
            }
            CalendarApiV4::EventAttendeeBatchDelete(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/attendees/batch_delete",
                    calendar_id, event_id
                )
            }
            CalendarApiV4::EventAttendeeChatMemberList(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/attendees/chat_member",
                    calendar_id, event_id
                )
            }

            // Freebusy
            CalendarApiV4::FreebusyQuery => "/open-apis/calendar/v4/freebusy/query".to_string(),

            // Setting
            CalendarApiV4::SettingGet => "/open-apis/calendar/v4/settings".to_string(),
            CalendarApiV4::SettingUpdate => "/open-apis/calendar/v4/settings".to_string(),

            // Timeoff Event
            CalendarApiV4::TimeoffEventCreate => {
                "/open-apis/calendar/v4/timeoff_events".to_string()
            }
            CalendarApiV4::TimeoffEventList => "/open-apis/calendar/v4/timeoff_events".to_string(),
            CalendarApiV4::TimeoffEventDelete(event_id) => {
                format!("/open-apis/calendar/v4/timeoff_events/{}", event_id)
            }

            // Exchange Binding
            CalendarApiV4::ExchangeBindingGet => {
                "/open-apis/calendar/v4/exchange_bindings".to_string()
            }
            CalendarApiV4::ExchangeBindingCreate => {
                "/open-apis/calendar/v4/exchange_bindings".to_string()
            }
            CalendarApiV4::ExchangeBindingDelete(exchange_id) => {
                format!("/open-apis/calendar/v4/exchange_bindings/{}", exchange_id)
            }

            // Meeting Chat
            CalendarApiV4::MeetingChatGet(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/meeting_chat",
                    calendar_id, event_id
                )
            }

            // Meeting Minute
            CalendarApiV4::MeetingMinuteGet(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/meeting_minute",
                    calendar_id, event_id
                )
            }
            CalendarApiV4::MeetingMinutePatch(calendar_id, event_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/events/{}/meeting_minute",
                    calendar_id, event_id
                )
            }

            // Subscription
            CalendarApiV4::CalendarSubscribe(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{}/subscribe", calendar_id)
            }
            CalendarApiV4::CalendarUnsubscribe(calendar_id) => {
                format!(
                    "/open-apis/calendar/v4/calendars/{}/unsubscribe",
                    calendar_id
                )
            }
        }
    }
}

// ==================== VC（视频会议）v1 ====================

/// VC API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum VcApiV1 {
    // ========== Room 会议室 ==========
    /// 创建会议室
    /// POST /open-apis/vc/v1/rooms
    RoomCreate,

    /// 获取会议室
    /// GET /open-apis/vc/v1/rooms/:room_id
    RoomGet(String),

    /// 删除会议室
    /// DELETE /open-apis/vc/v1/rooms/:room_id
    RoomDelete(String),

    /// 获取会议室列表
    /// GET /open-apis/vc/v1/rooms
    RoomList,

    /// 批量获取会议室
    /// POST /open-apis/vc/v1/rooms/batch_get
    RoomBatchGet,

    /// 更新会议室
    /// PATCH /open-apis/vc/v1/rooms/:room_id
    RoomPatch(String),

    // ========== Meeting 会议 ==========
    /// 获取会议
    /// GET /open-apis/vc/v1/meetings/:meeting_id
    MeetingGet(String),

    /// 邀请会议
    /// POST /open-apis/vc/v1/meetings/:meeting_id/invite
    MeetingInvite(String),

    /// 踢出会议
    /// POST /open-apis/vc/v1/meetings/:meeting_id/kickout
    MeetingKickout(String),

    /// 结束会议
    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/end
    MeetingEnd(String),

    /// 设置主持人
    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/set_host
    MeetingSetHost(String),

    /// 获取会议列表
    /// GET /open-apis/vc/v1/meetings
    MeetingList,

    /// 获取会议录制
    /// GET /open-apis/vc/v1/meetings/:meeting_id/recordings/:recording_id
    MeetingRecordingGet(String, String),

    /// 获取会议录制列表
    /// GET /open-apis/vc/v1/meetings/:meeting_id/recordings
    MeetingRecordingList(String),

    // ========== Reserve 预约 ==========
    /// 预约会议
    /// POST /open-apis/vc/v1/reserves
    ReserveCreate,

    /// 删除预约
    /// DELETE /open-apis/vc/v1/reserves/:reserve_id
    ReserveDelete(String),

    /// 获取预约
    /// GET /open-apis/vc/v1/reserves/:reserve_id
    ReserveGet(String),

    /// 获取预约活跃会议
    /// GET /open-apis/vc/v1/reserves/:reserve_id/active_meeting
    ReserveGetActiveMeeting(String),

    /// 更新预约
    /// PATCH /open-apis/vc/v1/reserves/:reserve_id
    ReservePatch(String),

    // ========== Report 报表 ==========
    /// 获取日报表
    /// GET /open-apis/vc/v1/reports/daily
    ReportDailyGet,

    /// 获取 Top 用户
    /// GET /open-apis/vc/v1/reports/top_user
    ReportTopUserGet,

    // ========== Export 导出 ==========
    /// 获取导出
    /// GET /open-apis/vc/v1/exports/:export_id
    ExportGet(String),

    /// 获取会议列表导出
    /// GET /open-apis/vc/v1/exports/meeting_list
    ExportMeetingList,

    /// 获取参会人列表导出
    /// GET /open-apis/vc/v1/exports/participant_list
    ExportParticipantList,

    /// 获取参会人质量列表导出
    /// GET /open-apis/vc/v1/exports/participant_quality_list
    ExportParticipantQualityList,

    // ========== Room Level 会议室层级 ==========
    /// 创建会议室层级
    /// POST /open-apis/vc/v1/room_levels
    RoomLevelCreate,

    /// 删除会议室层级
    /// DELETE /open-apis/vc/v1/room_levels/:room_level_id
    RoomLevelDelete(String),

    /// 获取会议室层级
    /// GET /open-apis/vc/v1/room_levels/:room_level_id
    RoomLevelGet(String),

    /// 获取会议室层级列表
    /// GET /open-apis/vc/v1/room_levels
    RoomLevelList,

    /// 更新会议室层级
    /// PATCH /open-apis/vc/v1/room_levels/:room_level_id
    RoomLevelPatch(String),

    // ========== Room Config 会议室配置 ==========
    /// 创建会议室配置
    /// POST /open-apis/vc/v1/room_configs
    RoomConfigCreate,

    /// 删除会议室配置
    /// DELETE /open-apis/vc/v1/room_configs/:room_config_id
    RoomConfigDelete(String),

    /// 获取会议室配置
    /// GET /open-apis/vc/v1/room_configs/:room_config_id
    RoomConfigGet(String),

    /// 获取会议室配置列表
    /// GET /open-apis/vc/v1/room_configs
    RoomConfigList,

    /// 更新会议室配置
    /// PATCH /open-apis/vc/v1/room_configs/:room_config_id
    RoomConfigPatch(String),

    // ========== Reserve Config 预约配置 ==========
    /// 创建预约配置
    /// POST /open-apis/vc/v1/reserve_configs
    ReserveConfigCreate,

    /// 删除预约配置
    /// DELETE /open-apis/vc/v1/reserve_configs/:reserve_config_id
    ReserveConfigDelete(String),

    /// 获取预约配置
    /// GET /open-apis/vc/v1/reserve_configs/:reserve_config_id
    ReserveConfigGet(String),

    /// 获取预约配置列表
    /// GET /open-apis/vc/v1/reserve_configs
    ReserveConfigList,

    /// 更新预约配置
    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id
    ReserveConfigPatch(String),

    // ========== Scope Config 范围配置 ==========
    /// 创建范围配置
    /// POST /open-apis/vc/v1/scope_config
    ScopeConfigCreate,

    /// 删除范围配置
    /// DELETE /open-apis/vc/v1/scope_config/:scope_config_id
    ScopeConfigDelete(String),

    /// 获取范围配置
    /// GET /open-apis/vc/v1/scope_config/:scope_config_id
    ScopeConfigGet(String),

    /// 获取范围配置列表
    /// GET /open-apis/vc/v1/scope_config
    ScopeConfigList,

    /// 更新范围配置
    /// PATCH /open-apis/vc/v1/scope_config/:scope_config_id
    ScopeConfigPatch(String),

    // ========== Alert 告警 ==========
    /// 获取告警列表
    /// GET /open-apis/vc/v1/alerts
    AlertList,

    /// 获取告警
    /// GET /open-apis/vc/v1/alerts/:alert_id
    AlertGet(String),

    // ========== List 列表 ==========
    /// 获取会议列表
    /// GET /open-apis/vc/v1/meeting_list
    MeetingListList,

    /// 获取参会人列表
    /// GET /open-apis/vc/v1/participant_list
    ParticipantListList,

    /// 获取参会人质量列表
    /// GET /open-apis/vc/v1/participant_quality_list
    ParticipantQualityListList,

    /// 获取资源预约列表
    /// GET /open-apis/vc/v1/resource_reservation_list
    ResourceReservationListList,
}

impl VcApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // Room
            VcApiV1::RoomCreate => "/open-apis/vc/v1/rooms".to_string(),
            VcApiV1::RoomGet(room_id) => format!("/open-apis/vc/v1/rooms/{}", room_id),
            VcApiV1::RoomDelete(room_id) => format!("/open-apis/vc/v1/rooms/{}", room_id),
            VcApiV1::RoomList => "/open-apis/vc/v1/rooms".to_string(),
            VcApiV1::RoomBatchGet => "/open-apis/vc/v1/rooms/batch_get".to_string(),
            VcApiV1::RoomPatch(room_id) => format!("/open-apis/vc/v1/rooms/{}", room_id),

            // Meeting
            VcApiV1::MeetingGet(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{}", meeting_id)
            }
            VcApiV1::MeetingInvite(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{}/invite", meeting_id)
            }
            VcApiV1::MeetingKickout(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{}/kickout", meeting_id)
            }
            VcApiV1::MeetingEnd(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{}/end", meeting_id)
            }
            VcApiV1::MeetingSetHost(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{}/set_host", meeting_id)
            }
            VcApiV1::MeetingList => "/open-apis/vc/v1/meetings".to_string(),
            VcApiV1::MeetingRecordingGet(meeting_id, recording_id) => {
                format!(
                    "/open-apis/vc/v1/meetings/{}/recordings/{}",
                    meeting_id, recording_id
                )
            }
            VcApiV1::MeetingRecordingList(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{}/recordings", meeting_id)
            }

            // Reserve
            VcApiV1::ReserveCreate => "/open-apis/vc/v1/reserves".to_string(),
            VcApiV1::ReserveDelete(reserve_id) => {
                format!("/open-apis/vc/v1/reserves/{}", reserve_id)
            }
            VcApiV1::ReserveGet(reserve_id) => {
                format!("/open-apis/vc/v1/reserves/{}", reserve_id)
            }
            VcApiV1::ReserveGetActiveMeeting(reserve_id) => {
                format!("/open-apis/vc/v1/reserves/{}/active_meeting", reserve_id)
            }
            VcApiV1::ReservePatch(reserve_id) => {
                format!("/open-apis/vc/v1/reserves/{}", reserve_id)
            }

            // Report
            VcApiV1::ReportDailyGet => "/open-apis/vc/v1/reports/daily".to_string(),
            VcApiV1::ReportTopUserGet => "/open-apis/vc/v1/reports/top_user".to_string(),

            // Export
            VcApiV1::ExportGet(export_id) => format!("/open-apis/vc/v1/exports/{}", export_id),
            VcApiV1::ExportMeetingList => "/open-apis/vc/v1/exports/meeting_list".to_string(),
            VcApiV1::ExportParticipantList => {
                "/open-apis/vc/v1/exports/participant_list".to_string()
            }
            VcApiV1::ExportParticipantQualityList => {
                "/open-apis/vc/v1/exports/participant_quality_list".to_string()
            }

            // Room Level
            VcApiV1::RoomLevelCreate => "/open-apis/vc/v1/room_levels".to_string(),
            VcApiV1::RoomLevelDelete(room_level_id) => {
                format!("/open-apis/vc/v1/room_levels/{}", room_level_id)
            }
            VcApiV1::RoomLevelGet(room_level_id) => {
                format!("/open-apis/vc/v1/room_levels/{}", room_level_id)
            }
            VcApiV1::RoomLevelList => "/open-apis/vc/v1/room_levels".to_string(),
            VcApiV1::RoomLevelPatch(room_level_id) => {
                format!("/open-apis/vc/v1/room_levels/{}", room_level_id)
            }

            // Room Config
            VcApiV1::RoomConfigCreate => "/open-apis/vc/v1/room_configs".to_string(),
            VcApiV1::RoomConfigDelete(room_config_id) => {
                format!("/open-apis/vc/v1/room_configs/{}", room_config_id)
            }
            VcApiV1::RoomConfigGet(room_config_id) => {
                format!("/open-apis/vc/v1/room_configs/{}", room_config_id)
            }
            VcApiV1::RoomConfigList => "/open-apis/vc/v1/room_configs".to_string(),
            VcApiV1::RoomConfigPatch(room_config_id) => {
                format!("/open-apis/vc/v1/room_configs/{}", room_config_id)
            }

            // Reserve Config
            VcApiV1::ReserveConfigCreate => "/open-apis/vc/v1/reserve_configs".to_string(),
            VcApiV1::ReserveConfigDelete(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{}", reserve_config_id)
            }
            VcApiV1::ReserveConfigGet(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{}", reserve_config_id)
            }
            VcApiV1::ReserveConfigList => "/open-apis/vc/v1/reserve_configs".to_string(),
            VcApiV1::ReserveConfigPatch(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{}", reserve_config_id)
            }

            // Scope Config
            VcApiV1::ScopeConfigCreate => "/open-apis/vc/v1/scope_config".to_string(),
            VcApiV1::ScopeConfigDelete(scope_config_id) => {
                format!("/open-apis/vc/v1/scope_config/{}", scope_config_id)
            }
            VcApiV1::ScopeConfigGet(scope_config_id) => {
                format!("/open-apis/vc/v1/scope_config/{}", scope_config_id)
            }
            VcApiV1::ScopeConfigList => "/open-apis/vc/v1/scope_config".to_string(),
            VcApiV1::ScopeConfigPatch(scope_config_id) => {
                format!("/open-apis/vc/v1/scope_config/{}", scope_config_id)
            }

            // Alert
            VcApiV1::AlertList => "/open-apis/vc/v1/alerts".to_string(),
            VcApiV1::AlertGet(alert_id) => format!("/open-apis/vc/v1/alerts/{}", alert_id),

            // List
            VcApiV1::MeetingListList => "/open-apis/vc/v1/meeting_list".to_string(),
            VcApiV1::ParticipantListList => "/open-apis/vc/v1/participant_list".to_string(),
            VcApiV1::ParticipantQualityListList => {
                "/open-apis/vc/v1/participant_quality_list".to_string()
            }
            VcApiV1::ResourceReservationListList => {
                "/open-apis/vc/v1/resource_reservation_list".to_string()
            }
        }
    }
}

// ==================== Meeting Room（会议室）old ====================

/// Meeting Room API 端点枚举（历史版本）
#[derive(Debug, Clone, PartialEq)]
pub enum MeetingRoomApi {
    // ========== Building 建筑物 ==========
    /// 创建建筑物
    /// POST /open-apis/meeting_room/buildings
    BuildingCreate,

    /// 删除建筑物
    /// DELETE /open-apis/meeting_room/buildings/:building_id
    BuildingDelete(String),

    /// 获取建筑物
    /// GET /open-apis/meeting_room/buildings/:building_id
    BuildingGet(String),

    /// 获取建筑物列表
    /// GET /open-apis/meeting_room/buildings
    BuildingList,

    /// 更新建筑物
    /// PATCH /open-apis/meeting_room/buildings/:building_id
    BuildingPatch(String),

    /// 批量获取建筑物
    /// POST /open-apis/meeting_room/buildings/batch_get
    BuildingBatchGet,

    /// 批量获取建筑物 ID
    /// POST /open-apis/meeting_room/buildings/batch_get_id
    BuildingBatchGetId,

    // ========== Room 会议室 ==========
    /// 创建会议室
    /// POST /open-apis/meeting_room/rooms
    RoomCreate,

    /// 删除会议室
    /// DELETE /open-apis/meeting_room/rooms/:room_id
    RoomDelete(String),

    /// 获取会议室
    /// GET /open-apis/meeting_room/rooms/:room_id
    RoomGet(String),

    /// 获取会议室列表
    /// GET /open-apis/meeting_room/rooms
    RoomList,

    /// 更新会议室
    /// PATCH /open-apis/meeting_room/rooms/:room_id
    RoomPatch(String),

    /// 批量获取会议室
    /// POST /open-apis/meeting_room/rooms/batch_get
    RoomBatchGet,

    /// 批量获取会议室 ID
    /// POST /open-apis/meeting_room/rooms/batch_get_id
    RoomBatchGetId,

    // ========== Instance 实例 ==========
    /// 批量获取忙碌情况
    /// POST /open-apis/meeting_room/instances/batch_get_freebusy
    InstanceBatchGetFreebusy,

    /// 回复实例
    /// POST /open-apis/meeting_room/instances/:instance_id/reply
    InstanceReply(String),

    // ========== 其他 ==========
    /// 获取国家列表
    /// GET /open-apis/meeting_room/countries
    CountryList,

    /// 获取地区列表
    /// GET /open-apis/meeting_room/districts
    DistrictList,

    /// 批量获取摘要
    /// POST /open-apis/meeting_room/rooms/batch_get_summary
    RoomBatchGetSummary,
}

impl MeetingRoomApi {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // Building
            MeetingRoomApi::BuildingCreate => "/open-apis/meeting_room/buildings".to_string(),
            MeetingRoomApi::BuildingDelete(building_id) => {
                format!("/open-apis/meeting_room/buildings/{}", building_id)
            }
            MeetingRoomApi::BuildingGet(building_id) => {
                format!("/open-apis/meeting_room/buildings/{}", building_id)
            }
            MeetingRoomApi::BuildingList => "/open-apis/meeting_room/buildings".to_string(),
            MeetingRoomApi::BuildingPatch(building_id) => {
                format!("/open-apis/meeting_room/buildings/{}", building_id)
            }
            MeetingRoomApi::BuildingBatchGet => {
                "/open-apis/meeting_room/buildings/batch_get".to_string()
            }
            MeetingRoomApi::BuildingBatchGetId => {
                "/open-apis/meeting_room/buildings/batch_get_id".to_string()
            }

            // Room
            MeetingRoomApi::RoomCreate => "/open-apis/meeting_room/rooms".to_string(),
            MeetingRoomApi::RoomDelete(room_id) => {
                format!("/open-apis/meeting_room/rooms/{}", room_id)
            }
            MeetingRoomApi::RoomGet(room_id) => {
                format!("/open-apis/meeting_room/rooms/{}", room_id)
            }
            MeetingRoomApi::RoomList => "/open-apis/meeting_room/rooms".to_string(),
            MeetingRoomApi::RoomPatch(room_id) => {
                format!("/open-apis/meeting_room/rooms/{}", room_id)
            }
            MeetingRoomApi::RoomBatchGet => "/open-apis/meeting_room/rooms/batch_get".to_string(),
            MeetingRoomApi::RoomBatchGetId => {
                "/open-apis/meeting_room/rooms/batch_get_id".to_string()
            }

            // Instance
            MeetingRoomApi::InstanceBatchGetFreebusy => {
                "/open-apis/meeting_room/instances/batch_get_freebusy".to_string()
            }
            MeetingRoomApi::InstanceReply(instance_id) => {
                format!("/open-apis/meeting_room/instances/{}/reply", instance_id)
            }

            // 其他
            MeetingRoomApi::CountryList => "/open-apis/meeting_room/countries".to_string(),
            MeetingRoomApi::DistrictList => "/open-apis/meeting_room/districts".to_string(),
            MeetingRoomApi::RoomBatchGetSummary => {
                "/open-apis/meeting_room/rooms/batch_get_summary".to_string()
            }
        }
    }
}
