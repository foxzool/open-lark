//! 日历服务端点常量定义

// ===== 日历基本操作 =====

/// 创建日历
pub const CALENDAR_CREATE: &str = "/open-apis/calendar/v4/calendars";

/// 获取日历详情
pub const CALENDAR_GET: &str = "/open-apis/calendar/v4/calendars/{calendar_id}";

/// 更新日历
pub const CALENDAR_UPDATE: &str = "/open-apis/calendar/v4/calendars/{calendar_id}";

/// 删除日历
pub const CALENDAR_DELETE: &str = "/open-apis/calendar/v4/calendars/{calendar_id}";

/// 获取日历列表
pub const CALENDAR_LIST: &str = "/open-apis/calendar/v4/calendars";

/// 获取主日历
pub const CALENDAR_PRIMARY: &str = "/open-apis/calendar/v4/calendars/primary";

/// 搜索日历
pub const CALENDAR_SEARCH: &str = "/open-apis/calendar/v4/calendars/{calendar_id}/search";

// ===== 日程操作 =====

/// 创建日程
pub const CALENDAR_EVENT_CREATE: &str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";

/// 获取日程详情
pub const CALENDAR_EVENT_GET: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

/// 更新日程
pub const CALENDAR_EVENT_UPDATE: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

/// 删除日程
pub const CALENDAR_EVENT_DELETE: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

/// 获取日程列表
pub const CALENDAR_EVENT_LIST: &str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";

/// 回复日程
pub const CALENDAR_EVENT_REPLY: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/reply";

/// 搜索日程
pub const CALENDAR_EVENT_SEARCH: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/search";

// ===== v4 兼容性别名 =====

/// v4 日历列表（兼容别名）
pub const CALENDAR_V4_CALENDARS: &str = "/open-apis/calendar/v4/calendars";

/// v4 日历操作（兼容别名）
pub const CALENDAR_V4_CALENDAR_OPERATION: &str = "/open-apis/calendar/v4/calendars/{calendar_id}";
