use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2VcMeetingStartedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2VcMeetingStartedV1Data,
}

pub(crate) struct P2VcMeetingStartedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingStartedV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2VcMeetingStartedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingStartedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2VcMeetingStartedV1 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2VcMeetingStartedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingStartedV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2VcMeetingStartedV1ProcessorImpl { f }
    }
}

/// 视频会议开始事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2VcMeetingStartedV1Data {
    /// 事件对象
    pub object: VcMeetingEventObject,
}

/// 视频会议事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct VcMeetingEventObject {
    /// 对象类型 (meeting)
    pub object_type: String,
    /// 会议信息
    pub meeting: VcMeeting,
}

/// 视频会议信息
#[derive(Debug, Serialize, Deserialize)]
pub struct VcMeeting {
    /// 会议ID
    pub meeting_id: String,
    /// 会议主题
    pub topic: String,
    /// 会议类型 (instant, scheduled, recurring)
    pub meeting_type: String,
    /// 会议状态 (started, ongoing, ended)
    pub status: String,
    /// 会议开始时间 (Unix时间戳，单位：秒)
    pub start_time: String,
    /// 预计结束时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 会议主持人
    pub host: MeetingParticipant,
    /// 会议参与者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<MeetingParticipant>>,
    /// 会议设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MeetingSettings>,
    /// 会议链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_url: Option<String>,
    /// 会议室信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_room: Option<MeetingRoom>,
    /// 关联的日历事件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_event_id: Option<String>,
    /// 录制信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<MeetingRecording>,
    /// 创建时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

/// 会议参与者
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingParticipant {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 参与者角色 (host, co_host, participant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 加入时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    /// 离开时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<String>,
    /// 参与状态 (joined, left, waiting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

/// 会议设置
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingSettings {
    /// 是否需要密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    /// 是否开启等候室
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting_room_enabled: Option<bool>,
    /// 是否允许录制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_allowed: Option<bool>,
    /// 是否自动录制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_recording: Option<bool>,
    /// 是否允许屏幕共享
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_sharing_allowed: Option<bool>,
    /// 音频设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_settings: Option<AudioSettings>,
    /// 视频设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_settings: Option<VideoSettings>,
}

/// 音频设置
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioSettings {
    /// 参与者加入时是否静音
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_on_entry: Option<bool>,
    /// 是否允许参与者取消静音
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unmute: Option<bool>,
}

/// 视频设置
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoSettings {
    /// 参与者加入时是否关闭视频
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_off_on_entry: Option<bool>,
    /// 视频质量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
}

/// 会议室信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingRoom {
    /// 会议室ID
    pub room_id: String,
    /// 会议室名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    /// 会议室位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// 会议室容量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}

/// 录制信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingRecording {
    /// 是否正在录制
    pub is_recording: bool,
    /// 录制开始时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_start_time: Option<String>,
    /// 录制文件类型 (video, audio, screen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_type: Option<String>,
    /// 录制质量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_quality: Option<String>,
}
