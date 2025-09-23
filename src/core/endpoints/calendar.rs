//! 日历服务端点常量定义

/// 获取日历列表
pub const CALENDAR_V4_CALENDARS: &str = "/open-apis/calendar/v4/calendars";

/// 创建日历
pub const CALENDAR_V4_CALENDARS_CREATE: &str = "/open-apis/calendar/v4/calendars";

/// 获取日历详情
pub const CALENDAR_V4_CALENDAR_GET: &str = "/open-apis/calendar/v4/calendars/{calendar_id}";

/// 创建日程
pub const CALENDAR_V4_EVENTS: &str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";

/// 获取日程列表
pub const CALENDAR_V4_EVENTS_LIST: &str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";

// TODO: 添加完整的日历端点列表
