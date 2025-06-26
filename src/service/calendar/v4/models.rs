use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, ResponseFormat},
};
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

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {}

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 基础请求结构
#[derive(Default, Clone)]
pub struct BaseCalendarRequest {
    pub api_req: ApiRequest,
}

/// 用户ID类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    /// Open ID
    OpenId,
    /// Union ID
    UnionId,
    /// User ID
    UserId,
}

impl std::fmt::Display for UserIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserIdType::OpenId => write!(f, "open_id"),
            UserIdType::UnionId => write!(f, "union_id"),
            UserIdType::UserId => write!(f, "user_id"),
        }
    }
}
