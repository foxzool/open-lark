//! Calendar服务数据模型
//!
//! 定义日历相关的数据结构，包括日历、日程、参与人等核心实体。

use serde::{Deserialize, Serialize};

/// 日历信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    /// 日历ID
    pub calendar_id: Option<String>,
    /// 日历标题
    pub summary: Option<String>,
    /// 日历描述
    pub description: Option<String>,
    /// 日历权限
    pub permissions: Option<CalendarPermission>,
    /// 日历颜色
    pub color: Option<i32>,
    /// 日历类型
    pub r#type: Option<CalendarType>,
    /// 日历摘要信息
    pub summary_info: Option<CalendarSummaryInfo>,
    /// 是否是主日历
    pub is_primary: Option<bool>,
    /// 角色
    pub role: Option<CalendarRole>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 是否已删除
    pub is_deleted: Option<bool>,
    /// 是否是第三方数据
    pub is_third_party: Option<bool>,
}

/// 日历权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarPermission {
    /// 是否可编辑
    pub access_role: Option<CalendarAccessRole>,
}

/// 日历访问角色
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarAccessRole {
    /// 未知
    Unknown,
    /// 无权限
    None,
    /// 忙闲信息
    FreeBusyReader,
    /// 只读
    Reader,
    /// 可编辑
    Writer,
    /// 所有者
    Owner,
}

/// 日历类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarType {
    /// 未知
    Unknown,
    /// 主日历
    Primary,
    /// 共享日历
    Shared,
    /// 谷歌日历
    Google,
    /// 资源日历
    Resource,
    /// Exchange日历
    Exchange,
}

/// 日历摘要信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarSummaryInfo {
    /// 日历颜色
    pub color: Option<i32>,
    /// 日历标题
    pub summary: Option<String>,
}

/// 日历角色
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarRole {
    /// 未知
    Unknown,
    /// 无权限
    None,
    /// 忙闲信息
    FreeBusyReader,
    /// 只读
    Reader,
    /// 可编辑
    Writer,
    /// 所有者
    Owner,
}

/// 日程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    /// 日程ID
    pub event_id: Option<String>,
    /// 组织者日程ID
    pub organizer_calendar_id: Option<String>,
    /// 日程标题
    pub summary: Option<String>,
    /// 日程描述
    pub description: Option<String>,
    /// 开始时间
    pub start_time: Option<TimeInfo>,
    /// 结束时间
    pub end_time: Option<TimeInfo>,
    /// 是否全天日程
    pub is_all_day: Option<bool>,
    /// 重复规则
    pub recurrence: Option<String>,
    /// 提醒设置
    pub reminders: Option<Vec<Reminder>>,
    /// 参与人
    pub attendees: Option<Vec<EventAttendee>>,
    /// 会议室
    pub meeting_rooms: Option<Vec<MeetingRoom>>,
    /// 位置
    pub location: Option<Location>,
    /// 日程颜色
    pub color: Option<i32>,
    /// 日程状态
    pub status: Option<EventStatus>,
    /// 是否是自由时间
    pub is_free_busy: Option<bool>,
    /// 创建人
    pub creator: Option<EventCreator>,
    /// 组织者
    pub organizer: Option<EventOrganizer>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
}

/// 时间信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInfo {
    /// 时间戳
    pub timestamp: Option<String>,
    /// 日期（全天日程）
    pub date: Option<String>,
    /// 时区
    pub timezone: Option<String>,
}

/// 提醒设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    /// 提醒分钟数
    pub minutes: Option<i32>,
}

/// 日程参与人
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttendee {
    /// 参与人类型
    pub r#type: Option<AttendeeType>,
    /// 参与人ID
    pub attendee_id: Option<String>,
    /// 参与人RSVP状态
    pub rsvp_status: Option<RsvpStatus>,
    /// 是否是可选参与人
    pub is_optional: Option<bool>,
    /// 是否是组织者
    pub is_organizer: Option<bool>,
    /// 是否是外部参与人
    pub is_external: Option<bool>,
    /// 显示名称
    pub display_name: Option<String>,
    /// 聊天ID
    pub chat_id: Option<String>,
    /// 房间ID
    pub room_id: Option<String>,
    /// 第三方邮箱
    pub third_party_email: Option<String>,
    /// 是否禁用状态同步
    pub operate_id: Option<String>,
    /// 资源自定义字段
    pub resource_customization: Option<Vec<ResourceCustomization>>,
}

/// 参与人类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttendeeType {
    /// 用户
    User,
    /// 聊天群
    Chat,
    /// 资源
    Resource,
    /// 第三方邮箱
    ThirdParty,
}

/// RSVP状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RsvpStatus {
    /// 需要回复
    NeedsAction,
    /// 接受
    Accept,
    /// 拒绝
    Decline,
    /// 暂定
    Tentative,
}

/// 会议室信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoom {
    /// 会议室ID
    pub room_id: Option<String>,
    /// 显示名称
    pub display_name: Option<String>,
    /// 会议室名称
    pub name: Option<String>,
    /// 容量
    pub capacity: Option<i32>,
    /// 楼层
    pub floor: Option<String>,
    /// 建筑
    pub building: Option<String>,
    /// 设备
    pub equipment: Option<Vec<String>>,
    /// 状态
    pub status: Option<MeetingRoomStatus>,
}

/// 位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// 位置名称
    pub name: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 纬度
    pub latitude: Option<f64>,
    /// 经度
    pub longitude: Option<f64>,
}

/// 日程状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventStatus {
    /// 暂定
    Tentative,
    /// 确认
    Confirmed,
    /// 取消
    Cancelled,
}

/// 日程创建人
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCreator {
    /// 创建人ID
    pub user_id: Option<String>,
    /// 显示名称
    pub display_name: Option<String>,
}

/// 日程组织者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOrganizer {
    /// 组织者ID
    pub user_id: Option<String>,
    /// 显示名称
    pub display_name: Option<String>,
}

/// 资源自定义字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCustomization {
    /// 索引ID
    pub index_id: Option<String>,
    /// 输入内容
    pub input_content: Option<String>,
    /// 选项
    pub options: Option<Vec<ResourceCustomizationOption>>,
}

/// 资源自定义选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCustomizationOption {
    /// 选项键
    pub option_key: Option<String>,
    /// 其他选项
    pub others_option: Option<String>,
}

/// 日历访问控制
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarAcl {
    /// ACL ID
    pub acl_id: Option<String>,
    /// 权限
    pub role: Option<CalendarRole>,
    /// 作用域
    pub scope: Option<AclScope>,
}

/// ACL作用域
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclScope {
    /// 作用域类型
    pub r#type: Option<AclScopeType>,
    /// 用户ID
    pub user_id: Option<String>,
}

/// ACL作用域类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AclScopeType {
    /// 用户
    User,
}

// ==================== 忙闲时间查询模型 ====================

/// 忙闲状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreeBusyStatus {
    /// 未知
    Unknown,
    /// 空闲
    Free,
    /// 忙碌
    Busy,
    /// 暂定
    Tentative,
    /// 外出
    OutOfOffice,
}

/// 忙闲时间范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusyTimeRange {
    /// 开始时间
    pub start_time: TimeInfo,
    /// 结束时间
    pub end_time: TimeInfo,
    /// 状态
    pub status: FreeBusyStatus,
}

/// 用户忙闲时间数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFreeBusyData {
    /// 用户ID
    pub user_id: String,
    /// 时间范围列表
    pub time_ranges: Vec<FreeBusyTimeRange>,
}

/// 用户忙闲时间响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFreeBusyResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<UserFreeBusyData>,
}

/// 多用户忙闲时间数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersFreeBusyData {
    /// 用户忙闲数据列表
    pub users: Vec<UserFreeBusyData>,
}

/// 多用户忙闲时间响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersFreeBusyResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<UsersFreeBusyData>,
}

/// 获取用户忙闲时间请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserFreeBusyRequest {
    /// 用户ID
    pub user_id: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 时区
    pub timezone: Option<String>,
}

/// 获取多用户忙闲时间请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUsersFreeBusyRequest {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 时区
    pub timezone: Option<String>,
}

// ==================== 参与人管理模型 ====================

/// 日程参与人数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttendeesData {
    /// 参与人列表
    pub attendees: Vec<EventAttendee>,
    /// 总数
    pub total: i32,
}

/// 日程参与人响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttendeesResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<EventAttendeesData>,
}

/// 获取日程参与人请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventAttendeesRequest {
    /// 日程ID
    pub event_id: String,
    /// 日历ID
    pub calendar_id: Option<String>,
}

/// 添加日程参与人请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEventAttendeesRequest {
    /// 日程ID
    pub event_id: String,
    /// 参与人ID列表
    pub attendee_ids: Vec<String>,
    /// 是否发送通知
    pub send_notifications: Option<bool>,
}

/// 删除日程参与人请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveEventAttendeesRequest {
    /// 日程ID
    pub event_id: String,
    /// 参与人ID列表
    pub attendee_ids: Vec<String>,
}

// ==================== 会议室管理模型 ====================

/// 会议室状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeetingRoomStatus {
    /// 未知
    Unknown,
    /// 可用
    Available,
    /// 已占用
    Occupied,
    /// 维护中
    Maintenance,
    /// 禁用
    Disabled,
}


/// 会议室数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoomsData {
    /// 会议室列表
    pub rooms: Vec<MeetingRoom>,
    /// 总数
    pub total: i32,
}

/// 会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoomsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<MeetingRoomsData>,
}

/// 获取会议室列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMeetingRoomsRequest {
    /// 楼层
    pub floor: Option<String>,
    /// 建筑
    pub building: Option<String>,
    /// 容量
    pub capacity_min: Option<i32>,
    pub capacity_max: Option<i32>,
    /// 状态
    pub status: Option<MeetingRoomStatus>,
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 预订状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookingStatus {
    /// 未知
    Unknown,
    /// 待确认
    Pending,
    /// 已确认
    Confirmed,
    /// 已取消
    Cancelled,
    /// 已拒绝
    Rejected,
}

/// 会议室预订数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeetingRoomData {
    /// 预订ID
    pub booking_id: Option<String>,
    /// 会议室ID
    pub room_id: Option<String>,
    /// 开始时间
    pub start_time: TimeInfo,
    /// 结束时间
    pub end_time: TimeInfo,
    /// 预订人
    pub booker: Option<EventCreator>,
    /// 预订状态
    pub status: Option<BookingStatus>,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 会议室预订响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeetingRoomResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<BookMeetingRoomData>,
}

/// 预订会议室请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeetingRoomRequest {
    /// 会议室ID
    pub room_id: String,
    /// 开始时间
    pub start_time: TimeInfo,
    /// 结束时间
    pub end_time: TimeInfo,
    /// 预订主题
    pub subject: Option<String>,
    /// 描述
    pub description: Option<String>,
}

// ==================== 日历订阅与共享模型 ====================

/// 订阅状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// 未知
    Unknown,
    /// 活跃
    Active,
    /// 已暂停
    Suspended,
    /// 已取消
    Cancelled,
}

/// 日历订阅数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeCalendarData {
    /// 订阅ID
    pub subscription_id: Option<String>,
    /// 日历ID
    pub calendar_id: Option<String>,
    /// 订阅者ID
    pub subscriber_id: Option<String>,
    /// 订阅状态
    pub status: Option<SubscriptionStatus>,
    /// 权限角色
    pub role: Option<CalendarRole>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 日历订阅响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeCalendarResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SubscribeCalendarData>,
}

/// 取消订阅数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeCalendarData {
    /// 订阅ID
    pub subscription_id: Option<String>,
    /// 日历ID
    pub calendar_id: Option<String>,
    /// 是否已取消
    pub unsubscribed: Option<bool>,
}

/// 取消订阅响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeCalendarResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<UnsubscribeCalendarData>,
}

/// 日历订阅列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarSubscriptionsData {
    /// 订阅列表
    pub subscriptions: Vec<SubscribeCalendarData>,
    /// 总数
    pub total: i32,
}

/// 日历订阅列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarSubscriptionsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CalendarSubscriptionsData>,
}

/// 订阅日历请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeCalendarRequest {
    /// 日历ID
    pub calendar_id: String,
    /// 订阅者ID
    pub subscriber_id: Option<String>,
    /// 权限角色
    pub role: Option<CalendarRole>,
}

/// 取消订阅日历请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeCalendarRequest {
    /// 日历ID
    pub calendar_id: String,
    /// 订阅ID
    pub subscription_id: Option<String>,
}

/// 获取日历订阅列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCalendarSubscriptionsRequest {
    /// 订阅者ID
    pub subscriber_id: Option<String>,
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {}