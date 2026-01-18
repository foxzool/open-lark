//! OpenLark Meeting 服务端点定义
//!
//! 本模块包含日历（calendar）、视频会议（vc）、会议室（meeting_room）相关 API 端点常量。
//! 端点常量优先提供"资源根路径"，具体接口在实现中通过拼接/format 构造。

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

// ==================== 具体接口常量 ====================

// ========== Calendar 具体接口 ==========
/// 创建日程
pub const CALENDAR_V4_EVENT_CREATE: &str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";
/// 获取日程
pub const CALENDAR_V4_EVENT_GET: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";
/// 更新日程
pub const CALENDAR_V4_EVENT_PATCH: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";
/// 删除日程
pub const CALENDAR_V4_EVENT_DELETE: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";
/// 获取日程列表
pub const CALENDAR_V4_EVENT_LIST: &str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";
/// 批量获取日程
pub const CALENDAR_V4_EVENT_BATCH_GET: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/batch_get";
/// 回复日程
pub const CALENDAR_V4_EVENT_REPLY: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/attendees/chat_member";

// ========== VC 具体接口 ==========
/// 创建会议室
pub const VC_V1_ROOM_CREATE: &str = "/open-apis/vc/v1/rooms";
/// 获取会议室
pub const VC_V1_ROOM_GET: &str = "/open-apis/vc/v1/rooms/{room_id}";
/// 删除会议室
pub const VC_V1_ROOM_DELETE: &str = "/open-apis/vc/v1/rooms/{room_id}";
/// 更新会议室
pub const VC_V1_ROOM_PATCH: &str = "/open-apis/vc/v1/rooms/{room_id}";
/// 获取会议室列表
pub const VC_V1_ROOM_LIST: &str = "/open-apis/vc/v1/rooms";
/// 批量获取会议室
pub const VC_V1_ROOM_BATCH_GET: &str = "/open-apis/vc/v1/rooms/batch_get";

/// 创建会议室层级
pub const VC_V1_ROOM_LEVEL_CREATE: &str = "/open-apis/vc/v1/room_levels";
/// 获取会议室层级
pub const VC_V1_ROOM_LEVEL_GET: &str = "/open-apis/vc/v1/room_levels/{room_level_id}";
/// 删除会议室层级
pub const VC_V1_ROOM_LEVEL_DELETE: &str = "/open-apis/vc/v1/room_levels/{room_level_id}";
/// 更新会议室层级
pub const VC_V1_ROOM_LEVEL_PATCH: &str = "/open-apis/vc/v1/room_levels/{room_level_id}";
/// 获取会议室层级列表
pub const VC_V1_ROOM_LEVEL_LIST: &str = "/open-apis/vc/v1/room_levels";

/// 创建会议室配置
pub const VC_V1_ROOM_CONFIG_CREATE: &str = "/open-apis/vc/v1/room_configs";
/// 获取会议室配置
pub const VC_V1_ROOM_CONFIG_GET: &str = "/open-apis/vc/v1/room_configs/{room_config_id}";
/// 删除会议室配置
pub const VC_V1_ROOM_CONFIG_DELETE: &str = "/open-apis/vc/v1/room_configs/{room_config_id}";
/// 更新会议室配置
pub const VC_V1_ROOM_CONFIG_PATCH: &str = "/open-apis/vc/v1/room_configs/{room_config_id}";
/// 获取会议室配置列表
pub const VC_V1_ROOM_CONFIG_LIST: &str = "/open-apis/vc/v1/room_configs";

/// 设置会议室访问码
pub const VC_V1_ROOM_CONFIG_SET_ACCESS_CODE: &str =
    "/open-apis/vc/v1/room_configs/set_room_access_code";
/// 设置白板访问码
pub const VC_V1_ROOM_CONFIG_SET_CHECKBOARD_ACCESS_CODE: &str =
    "/open-apis/vc/v1/room_configs/set_checkboard_access_code";

/// 预约会议
pub const VC_V1_RESERVE_CREATE: &str = "/open-apis/vc/v1/reserves";
/// 获取预约
pub const VC_V1_RESERVE_GET: &str = "/open-apis/vc/v1/reserves/{reserve_id}";
/// 更新预约
pub const VC_V1_RESERVE_PATCH: &str = "/open-apis/vc/v1/reserves/{reserve_id}";
/// 删除预约
pub const VC_V1_RESERVE_DELETE: &str = "/open-apis/vc/v1/reserves/{reserve_id}";
/// 获取预约活跃会议
pub const VC_V1_RESERVE_GET_ACTIVE_MEETING: &str =
    "/open-apis/vc/v1/reserves/{reserve_id}/active_meeting";

/// 获取会议
pub const VC_V1_MEETING_GET: &str = "/open-apis/vc/v1/meetings/{meeting_id}";
/// 邀请会议
pub const VC_V1_MEETING_INVITE: &str = "/open-apis/vc/v1/meetings/{meeting_id}/invite";
/// 踢出会议
pub const VC_V1_MEETING_KICKOUT: &str = "/open-apis/vc/v1/meetings/{meeting_id}/kickout";
/// 结束会议
pub const VC_V1_MEETING_END: &str = "/open-apis/vc/v1/meetings/{meeting_id}/end";
/// 设置主持人
pub const VC_V1_MEETING_SET_HOST: &str = "/open-apis/vc/v1/meetings/{meeting_id}/set_host";

/// 获取导出
pub const VC_V1_EXPORT_GET: &str = "/open-apis/vc/v1/exports/{export_id}";
/// 下载导出
pub const VC_V1_EXPORT_DOWNLOAD: &str = "/open-apis/vc/v1/exports/{export_id}/download";
/// 获取会议列表导出
pub const VC_V1_EXPORT_MEETING_LIST: &str = "/open-apis/vc/v1/exports/meeting_list";
/// 获取参会人列表导出
pub const VC_V1_EXPORT_PARTICIPANT_LIST: &str = "/open-apis/vc/v1/exports/participant_list";
/// 获取参会人质量列表导出
pub const VC_V1_EXPORT_PARTICIPANT_QUALITY_LIST: &str =
    "/open-apis/vc/v1/exports/participant_quality_list";
/// 获取资源预约列表导出
pub const VC_V1_EXPORT_RESOURCE_RESERVATION_LIST: &str =
    "/open-apis/vc/v1/exports/resource_reservation_list";

/// 获取日报表
pub const VC_V1_REPORT_DAILY_GET: &str = "/open-apis/vc/v1/reports/daily";
/// 获取 Top 用户
pub const VC_V1_REPORT_TOP_USER_GET: &str = "/open-apis/vc/v1/reports/top_user";

/// 获取告警列表
pub const VC_V1_ALERT_LIST: &str = "/open-apis/vc/v1/alerts";

/// 获取预约配置
pub const VC_V1_RESERVE_CONFIG_GET: &str = "/open-apis/vc/v1/reserve_configs/{reserve_config_id}";
/// 更新预约配置
pub const VC_V1_RESERVE_CONFIG_PATCH: &str = "/open-apis/vc/v1/reserve_configs/{reserve_config_id}";

/// 获取预约配置表单
pub const VC_V1_RESERVE_CONFIG_FORM_GET: &str =
    "/open-apis/vc/v1/reserve_configs/{reserve_config_id}/form";
/// 更新预约配置表单
pub const VC_V1_RESERVE_CONFIG_FORM_PATCH: &str =
    "/open-apis/vc/v1/reserve_configs/{reserve_config_id}/form";

/// 获取预约配置范围
pub const VC_V1_RESERVE_CONFIG_RESERVE_SCOPE: &str =
    "/open-apis/vc/v1/reserve_configs/{reserve_config_id}/reserve_scope";

/// 禁用通知
pub const VC_V1_RESERVE_CONFIG_DISABLE_INFORM_GET: &str =
    "/open-apis/vc/v1/reserve_configs/{reserve_config_id}/disable_inform";
/// 更新禁用通知
pub const VC_V1_RESERVE_CONFIG_DISABLE_INFORM_PATCH: &str =
    "/open-apis/vc/v1/reserve_configs/{reserve_config_id}/disable_inform";

/// 获取管理员
pub const VC_V1_RESERVE_CONFIG_ADMIN_GET: &str =
    "/open-apis/vc/v1/reserve_configs/{reserve_config_id}/admin";
/// 更新管理员
pub const VC_V1_RESERVE_CONFIG_ADMIN_PATCH: &str =
    "/open-apis/vc/v1/reserve_configs/{reserve_config_id}/admin";

// ========== Meeting Room 具体接口 ==========
/// 创建建筑物
pub const MEETING_ROOM_BUILDING_CREATE: &str = "/open-apis/meeting_room/buildings";
/// 获取建筑物
pub const MEETING_ROOM_BUILDING_GET: &str = "/open-apis/meeting_room/buildings/{building_id}";
/// 更新建筑物
pub const MEETING_ROOM_BUILDING_PATCH: &str = "/open-apis/meeting_room/buildings/{building_id}";
/// 删除建筑物
pub const MEETING_ROOM_BUILDING_DELETE: &str = "/open-apis/meeting_room/buildings/{building_id}";
/// 获取建筑物列表
pub const MEETING_ROOM_BUILDING_LIST: &str = "/open-apis/meeting_room/buildings";
/// 批量获取建筑物
pub const MEETING_ROOM_BUILDING_BATCH_GET: &str = "/open-apis/meeting_room/buildings/batch_get";
/// 批量获取建筑物 ID
pub const MEETING_ROOM_BUILDING_BATCH_GET_ID: &str =
    "/open-apis/meeting_room/buildings/batch_get_id";

/// 创建会议室
pub const MEETING_ROOM_ROOM_CREATE: &str = "/open-apis/meeting_room/rooms";
/// 获取会议室
pub const MEETING_ROOM_ROOM_GET: &str = "/open-apis/meeting_room/rooms/{room_id}";
/// 更新会议室
pub const MEETING_ROOM_ROOM_PATCH: &str = "/open-apis/meeting_room/rooms/{room_id}";
/// 删除会议室
pub const MEETING_ROOM_ROOM_DELETE: &str = "/open-apis/meeting_room/rooms/{room_id}";
/// 获取会议室列表
pub const MEETING_ROOM_ROOM_LIST: &str = "/open-apis/meeting_room/rooms";
/// 批量获取会议室
pub const MEETING_ROOM_ROOM_BATCH_GET: &str = "/open-apis/meeting_room/rooms/batch_get";
/// 批量获取会议室 ID
pub const MEETING_ROOM_ROOM_BATCH_GET_ID: &str = "/open-apis/meeting_room/rooms/batch_get_id";

/// 批量获取摘要
pub const MEETING_ROOM_ROOM_BATCH_GET_SUMMARY: &str =
    "/open-apis/meeting_room/rooms/batch_get_summary";

/// 回复实例
pub const MEETING_ROOM_INSTANCE_REPLY: &str =
    "/open-apis/meeting_room/instances/{instance_id}/reply";

/// 批量获取忙碌情况
pub const MEETING_ROOM_INSTANCE_BATCH_GET_FREEBUSY: &str =
    "/open-apis/meeting_room/instances/batch_get_freebusy";

/// 获取国家列表
pub const MEETING_ROOM_COUNTRY_LIST: &str = "/open-apis/meeting_room/countries";

/// 获取地区列表
pub const MEETING_ROOM_DISTRICT_LIST: &str = "/open-apis/meeting_room/districts";
