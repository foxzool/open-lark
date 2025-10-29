// meeting_room模块的数据模型定义

use serde::{Deserialize, Serialize};

/// 建筑物信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    /// 建筑物ID
    #[serde(rename = "building_id")]
    pub building_id: String,
    /// 建筑物名称
    pub name: String,
    /// 建筑物地址
    pub address: Option<String>,
    /// 建筑物描述
    pub description: Option<String>,
    /// 楼层列表
    pub floors: Option<Vec<Floor>>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: Option<String>,
    /// 建筑物状态
    #[serde(rename = "building_status")]
    pub building_status: BuildingStatus,
}

/// 建筑物状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BuildingStatus {
    /// 启用
    Enabled,
    /// 禁用
    Disabled,
}

/// 楼层信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floor {
    /// 楼层ID
    #[serde(rename = "floor_id")]
    pub floor_id: String,
    /// 楼层名称
    pub name: String,
    /// 楼层号
    pub floor_number: i32,
    /// 楼层描述
    pub description: Option<String>,
    /// 楼层平面图URL
    #[serde(rename = "floor_plan_url")]
    pub floor_plan_url: Option<String>,
}

/// 会议室信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoom {
    /// 会议室ID
    #[serde(rename = "room_id")]
    pub room_id: String,
    /// 会议室名称
    pub name: String,
    /// 所属建筑物ID
    #[serde(rename = "building_id")]
    pub building_id: String,
    /// 所属楼层ID
    #[serde(rename = "floor_id")]
    pub floor_id: String,
    /// 会议室容量
    pub capacity: i32,
    /// 会议室面积（平方米）
    pub area: Option<f64>,
    /// 会议室描述
    pub description: Option<String>,
    /// 会议室设备列表
    pub equipment: Option<Vec<Equipment>>,
    /// 会议室状态
    #[serde(rename = "room_status")]
    pub room_status: RoomStatus,
    /// 会议室类型
    #[serde(rename = "room_type")]
    pub room_type: RoomType,
    /// 会议室图片URL
    #[serde(rename = "room_image_urls")]
    pub room_image_urls: Option<Vec<String>>,
    /// 会议室位置描述
    #[serde(rename = "location_description")]
    pub location_description: Option<String>,
    /// 预订规则
    #[serde(rename = "booking_rules")]
    pub booking_rules: Option<BookingRules>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: Option<String>,
}

/// 会议室状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RoomStatus {
    /// 可用
    Available,
    /// 维护中
    UnderMaintenance,
    /// 已禁用
    Disabled,
    /// 已删除
    Deleted,
}

/// 会议室类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RoomType {
    /// 普通会议室
    Normal,
    /// 视频会议室
    VideoConference,
    /// 培训室
    Training,
    /// 休息室
    Lounge,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    /// 设备ID
    #[serde(rename = "equipment_id")]
    pub equipment_id: String,
    /// 设备名称
    pub name: String,
    /// 设备类型
    #[serde(rename = "equipment_type")]
    pub equipment_type: EquipmentType,
    /// 设备状态
    #[serde(rename = "equipment_status")]
    pub equipment_status: EquipmentStatus,
    /// 设备描述
    pub description: Option<String>,
}

/// 设备类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EquipmentType {
    /// 投影仪
    Projector,
    /// 白板
    Whiteboard,
    /// 电视
    TV,
    /// 视频会议设备
    VideoConference,
    /// 电话会议设备
    AudioConference,
    /// 空调
    AirConditioner,
    /// 网络
    Network,
}

/// 设备状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EquipmentStatus {
    /// 正常
    Normal,
    /// 故障
    Faulty,
    /// 维护中
    UnderMaintenance,
}

/// 预订规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingRules {
    /// 最短预订时长（分钟）
    #[serde(rename = "min_booking_duration_minutes")]
    pub min_booking_duration_minutes: i32,
    /// 最长预订时长（分钟）
    #[serde(rename = "max_booking_duration_minutes")]
    pub max_booking_duration_minutes: i32,
    /// 提前预订时间限制（小时）
    #[serde(rename = "advance_booking_limit_hours")]
    pub advance_booking_limit_hours: i32,
    /// 是否需要审批
    #[serde(rename = "requires_approval")]
    pub requires_approval: bool,
    /// 可预订时间范围
    #[serde(rename = "booking_time_range")]
    pub booking_time_range: Option<BookingTimeRange>,
    /// 重复预订规则
    #[serde(rename = "recurring_booking_rules")]
    pub recurring_booking_rules: Option<RecurringBookingRules>,
}

/// 预订时间范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingTimeRange {
    /// 开始时间
    #[serde(rename = "start_time")]
    pub start_time: String,
    /// 结束时间
    #[serde(rename = "end_time")]
    pub end_time: String,
}

/// 重复预订规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringBookingRules {
    /// 是否允许重复预订
    #[serde(rename = "allow_recurring")]
    pub allow_recurring: bool,
    /// 最大重复次数
    #[serde(rename = "max_recurrence_count")]
    pub max_recurrence_count: i32,
    /// 重复间隔类型
    #[serde(rename = "recurrence_interval_type")]
    pub recurrence_interval_type: RecurrenceIntervalType,
}

/// 重复间隔类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecurrenceIntervalType {
    /// 每天
    Daily,
    /// 每周
    Weekly,
    /// 每月
    Monthly,
}

/// 会议室忙闲信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusyInfo {
    /// 会议室ID
    #[serde(rename = "room_id")]
    pub room_id: String,
    /// 会议室名称
    pub name: String,
    /// 忙闲时间段列表
    #[serde(rename = "time_slots")]
    pub time_slots: Vec<TimeSlot>,
}

/// 时间段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    /// 开始时间
    #[serde(rename = "start_time")]
    pub start_time: String,
    /// 结束时间
    #[serde(rename = "end_time")]
    pub end_time: String,
    /// 状态
    pub status: TimeSlotStatus,
    /// 预订人信息
    #[serde(rename = "booker")]
    pub booker: Option<BookerInfo>,
    /// 会议主题
    #[serde(rename = "meeting_subject")]
    pub meeting_subject: Option<String>,
}

/// 时间段状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TimeSlotStatus {
    /// 可用
    Available,
    /// 已预订
    Booked,
    /// 维护中
    Maintenance,
}

/// 预订人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookerInfo {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名
    #[serde(rename = "user_name")]
    pub user_name: Option<String>,
    /// 用户头像
    #[serde(rename = "avatar")]
    pub avatar: Option<String>,
}

/// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoomResponse<T> {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}

/// 分页响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedMeetingRoomResponse<T> {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<PaginatedMeetingRoomData<T>>,
}

/// 分页数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedMeetingRoomData<T> {
    /// 数据列表
    pub items: Option<Vec<T>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 总数
    pub total: Option<i32>,
}