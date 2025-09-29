use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2VcMeetingEndedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2VcMeetingEndedV1Data,
}

pub(crate) struct P2VcMeetingEndedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingEndedV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2VcMeetingEndedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingEndedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2VcMeetingEndedV1 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2VcMeetingEndedV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingEndedV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2VcMeetingEndedV1ProcessorImpl { f }
    }
}

/// 视频会议结束事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2VcMeetingEndedV1Data {
    /// 事件对象
    pub object: VcMeetingEventObject,
    /// 结束前的会议状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_status: Option<String>,
}

/// 视频会议事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct VcMeetingEventObject {
    /// 对象类型 (meeting)
    pub object_type: String,
    /// 会议信息
    pub meeting: EndedVcMeeting,
}

/// 结束的视频会议信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EndedVcMeeting {
    /// 会议ID
    pub meeting_id: String,
    /// 会议主题
    pub topic: String,
    /// 会议类型 (instant, scheduled, recurring)
    pub meeting_type: String,
    /// 会议状态 (ended)
    pub status: String,
    /// 会议开始时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 会议结束时间 (Unix时间戳，单位：秒)
    pub end_time: String,
    /// 会议持续时间（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// 会议主持人
    pub host: MeetingParticipant,
    /// 会议参与者统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_stats: Option<ParticipantStats>,
    /// 最终参与者列表
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
    /// 录制信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<MeetingRecording>,
    /// 结束原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<MeetingEndReason>,
    /// 会议总结
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_summary: Option<MeetingSummary>,
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
    /// 参与总时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

/// 参与者统计信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantStats {
    /// 最大同时在线人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<i32>,
    /// 总加入人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_participants: Option<i32>,
    /// 平均参与时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_duration: Option<i64>,
    /// 实际参与人数（去除重复加入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_participants: Option<i32>,
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
    /// 是否进行了录制
    pub was_recorded: bool,
    /// 录制开始时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_start_time: Option<String>,
    /// 录制结束时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_end_time: Option<String>,
    /// 录制文件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_files: Option<Vec<RecordingFile>>,
    /// 录制总大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
}

/// 录制文件信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingFile {
    /// 文件ID
    pub file_id: String,
    /// 文件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件类型 (video, audio, screen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 文件大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 录制质量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// 文件URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

/// 会议结束原因
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingEndReason {
    /// 结束类型 (host_ended, timeout, system_ended, scheduled_end)
    pub end_type: String,
    /// 结束操作者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_by: Option<String>,
    /// 结束原因描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 会议总结
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingSummary {
    /// 会议评分（1-5分）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    /// 网络质量统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_quality: Option<NetworkQuality>,
    /// 主要议题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_topics: Option<Vec<String>>,
    /// 会议费用（如果适用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<MeetingCost>,
}

/// 网络质量统计
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkQuality {
    /// 平均网络质量（1-5分，5为最好）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_quality: Option<f64>,
    /// 网络中断次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruptions: Option<i32>,
    /// 总延迟（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_latency: Option<i32>,
}

/// 会议费用信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingCost {
    /// 费用金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// 货币单位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// 计费方式 (per_minute, per_participant, flat_rate)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
}
