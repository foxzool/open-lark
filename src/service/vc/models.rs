use serde::{Deserialize, Serialize};

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserIdType {
    /// 用户ID
    #[serde(rename = "user_id")]
    UserId,
    /// union_id
    #[serde(rename = "union_id")]
    UnionId,
    /// open_id
    #[serde(rename = "open_id")]
    OpenId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
}

/// 会议室ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoomIdType {
    /// 会议室ID
    #[serde(rename = "room_id")]
    RoomId,
    /// 会议室名称
    #[serde(rename = "omm_room_id")]
    OmmRoomId,
}

impl RoomIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoomIdType::RoomId => "room_id",
            RoomIdType::OmmRoomId => "omm_room_id",
        }
    }
}

/// 会议状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeetingStatus {
    /// 未开始
    #[serde(rename = "not_started")]
    NotStarted,
    /// 进行中
    #[serde(rename = "in_progress")]
    InProgress,
    /// 已结束
    #[serde(rename = "ended")]
    Ended,
    /// 已取消
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// 会议类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeetingType {
    /// 即时会议
    #[serde(rename = "instant")]
    Instant,
    /// 预约会议
    #[serde(rename = "scheduled")]
    Scheduled,
    /// 周期性会议
    #[serde(rename = "recurring")]
    Recurring,
}

/// 预约会议信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reserve {
    /// 预约ID
    pub id: String,
    /// 会议主题
    pub topic: String,
    /// 会议号
    pub meeting_no: String,
    /// 会议密码
    pub password: Option<String>,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 主持人
    pub host_user: Option<UserInfo>,
    /// 状态
    pub status: MeetingStatus,
    /// 会议类型
    pub meeting_type: MeetingType,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 会议信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    /// 会议ID
    pub id: String,
    /// 会议主题
    pub topic: String,
    /// 会议号
    pub meeting_no: String,
    /// 会议密码
    pub password: Option<String>,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: Option<String>,
    /// 主持人
    pub host_user: Option<UserInfo>,
    /// 状态
    pub status: MeetingStatus,
    /// 参会人数
    pub participant_count: Option<i32>,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 用户头像
    pub avatar_url: Option<String>,
}

/// 会议室信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    /// 会议室ID
    pub room_id: String,
    /// 会议室名称
    pub name: String,
    /// 会议室描述
    pub description: Option<String>,
    /// 会议室容量
    pub capacity: Option<i32>,
    /// 会议室位置
    pub location: Option<String>,
    /// 会议室状态
    pub status: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 录制信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recording {
    /// 录制ID
    pub recording_id: String,
    /// 会议ID
    pub meeting_id: String,
    /// 录制标题
    pub title: Option<String>,
    /// 录制时长
    pub duration: Option<i32>,
    /// 录制大小
    pub size: Option<i64>,
    /// 录制状态
    pub status: Option<String>,
    /// 录制开始时间
    pub start_time: Option<String>,
    /// 录制结束时间
    pub end_time: Option<String>,
}
