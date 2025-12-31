//! OpenLark Meeting 服务端点定义
//!
//! 本模块包含日历（calendar）、视频会议（vc）、会议室（meeting_room）相关 API 端点常量。
//! 端点常量优先提供“资源根路径”，具体接口在实现中通过拼接/format 构造。

// ==================== Calendar（日历）v4 ====================

/// 日历（calendar）资源根路径
pub const CALENDAR_V4_CALENDARS: &str = "/open-apis/calendar/v4/calendars";

/// 忙闲（freebusy）资源根路径
pub const CALENDAR_V4_FREEBUSY: &str = "/open-apis/calendar/v4/freebusy";

/// 日历设置（settings）资源根路径
pub const CALENDAR_V4_SETTINGS: &str = "/open-apis/calendar/v4/settings";

/// 请假日程（timeoff_events）资源根路径
pub const CALENDAR_V4_TIMEOFF_EVENTS: &str = "/open-apis/calendar/v4/timeoff_events";

/// Exchange 绑定（exchange_bindings）资源根路径
pub const CALENDAR_V4_EXCHANGE_BINDINGS: &str = "/open-apis/calendar/v4/exchange_bindings";

// ==================== VC（视频会议）v1 ====================
/// 会议预约（reserves）资源根路径
pub const VC_V1_RESERVES: &str = "/open-apis/vc/v1/reserves";

/// 会议（meetings）资源根路径
pub const VC_V1_MEETINGS: &str = "/open-apis/vc/v1/meetings";

/// 会议报告（reports）资源根路径
pub const VC_V1_REPORTS: &str = "/open-apis/vc/v1/reports";

/// 导出任务（exports）资源根路径
pub const VC_V1_EXPORTS: &str = "/open-apis/vc/v1/exports";

/// 会议室层级（room_levels）资源根路径
pub const VC_V1_ROOM_LEVELS: &str = "/open-apis/vc/v1/room_levels";

/// 会议室（rooms）资源根路径
pub const VC_V1_ROOMS: &str = "/open-apis/vc/v1/rooms";

/// 会议室配置（scope_config）资源根路径
pub const VC_V1_SCOPE_CONFIG: &str = "/open-apis/vc/v1/scope_config";

/// 会议室预定配置（reserve_configs）资源根路径
pub const VC_V1_RESERVE_CONFIGS: &str = "/open-apis/vc/v1/reserve_configs";

/// 会议室部署/范围配置（room_configs）资源根路径
pub const VC_V1_ROOM_CONFIGS: &str = "/open-apis/vc/v1/room_configs";

/// 设备告警（alerts）资源根路径
pub const VC_V1_ALERTS: &str = "/open-apis/vc/v1/alerts";

/// 会议明细（meeting_list）
pub const VC_V1_MEETING_LIST: &str = "/open-apis/vc/v1/meeting_list";

/// 参会人明细（participant_list）
pub const VC_V1_PARTICIPANT_LIST: &str = "/open-apis/vc/v1/participant_list";

/// 参会人会议质量数据（participant_quality_list）
pub const VC_V1_PARTICIPANT_QUALITY_LIST: &str = "/open-apis/vc/v1/participant_quality_list";

/// 会议室预定数据（resource_reservation_list）
pub const VC_V1_RESOURCE_RESERVATION_LIST: &str = "/open-apis/vc/v1/resource_reservation_list";

// ==================== Meeting Room（会议室）old ====================
/// 会议室（meeting_room）资源根路径
pub const MEETING_ROOM: &str = "/open-apis/meeting_room";
