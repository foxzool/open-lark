use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2VcMeetingParticipantJoinedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2VcMeetingParticipantJoinedV1Data,
}

pub(crate) struct P2VcMeetingParticipantJoinedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingParticipantJoinedV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2VcMeetingParticipantJoinedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingParticipantJoinedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2VcMeetingParticipantJoinedV1 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2VcMeetingParticipantJoinedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingParticipantJoinedV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2VcMeetingParticipantJoinedV1ProcessorImpl { f }
    }
}

/// 视频会议参与者加入事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2VcMeetingParticipantJoinedV1Data {
    /// 事件对象
    pub object: VcParticipantEventObject,
    /// 会议当前状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_status: Option<String>,
}

/// 视频会议参与者事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct VcParticipantEventObject {
    /// 对象类型 (meeting_participant)
    pub object_type: String,
    /// 参与者信息
    pub participant: JoinedMeetingParticipant,
    /// 会议基本信息
    pub meeting: MeetingBasicInfo,
}

/// 加入会议的参与者信息
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedMeetingParticipant {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 参与者角色 (host, co_host, participant, guest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 加入时间 (Unix时间戳，单位：秒)
    pub join_time: String,
    /// 加入方式 (dial_in, web, mobile, desktop, room_system)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_method: Option<String>,
    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,
    /// 网络信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_info: Option<NetworkInfo>,
    /// 初始状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_status: Option<ParticipantStatus>,
    /// 是否是重新加入
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rejoin: Option<bool>,
    /// 加入会议的邀请信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_info: Option<InvitationInfo>,
    /// 参与者权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ParticipantPermissions>,
}

/// 会议基本信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingBasicInfo {
    /// 会议ID
    pub meeting_id: String,
    /// 会议主题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// 会议类型 (instant, scheduled, recurring)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_type: Option<String>,
    /// 会议状态 (started, ongoing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 当前参与人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_participant_count: Option<i32>,
    /// 会议开始时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// 设备信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备类型 (web, desktop, mobile, room_system, phone)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// 操作系统 (windows, macos, ios, android, linux)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// 浏览器类型（web加入时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    /// 应用版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// 设备名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
}

/// 网络信息
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// IP地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// 网络类型 (wifi, cellular, ethernet, other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 网络质量 (excellent, good, fair, poor)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_quality: Option<String>,
    /// 延迟（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i32>,
    /// 地理位置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeographicLocation>,
}

/// 地理位置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct GeographicLocation {
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 地区/州
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// 参与者状态
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantStatus {
    /// 音频状态 (muted, unmuted, unavailable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_status: Option<String>,
    /// 视频状态 (on, off, unavailable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_status: Option<String>,
    /// 屏幕共享状态 (sharing, not_sharing, unavailable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_share_status: Option<String>,
    /// 举手状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_raised: Option<bool>,
    /// 在线状态 (online, away, busy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_status: Option<String>,
}

/// 邀请信息
#[derive(Debug, Serialize, Deserialize)]
pub struct InvitationInfo {
    /// 邀请人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,
    /// 邀请时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_time: Option<String>,
    /// 邀请方式 (email, sms, link, calendar, direct)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_method: Option<String>,
    /// 是否通过等候室加入
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_via_waiting_room: Option<bool>,
}

/// 参与者权限
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantPermissions {
    /// 是否可以开启/关闭麦克风
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_toggle_audio: Option<bool>,
    /// 是否可以开启/关闭视频
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_toggle_video: Option<bool>,
    /// 是否可以屏幕共享
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_share_screen: Option<bool>,
    /// 是否可以录制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_record: Option<bool>,
    /// 是否可以邀请其他人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_others: Option<bool>,
    /// 是否可以管理参与者（踢出、静音等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_participants: Option<bool>,
    /// 是否可以修改会议设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_modify_settings: Option<bool>,
    /// 是否可以查看参与者列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_participant_list: Option<bool>,
}
