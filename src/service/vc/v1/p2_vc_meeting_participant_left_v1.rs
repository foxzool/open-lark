use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2VcMeetingParticipantLeftV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2VcMeetingParticipantLeftV1Data,
}

pub(crate) struct P2VcMeetingParticipantLeftV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingParticipantLeftV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2VcMeetingParticipantLeftV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingParticipantLeftV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2VcMeetingParticipantLeftV1 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2VcMeetingParticipantLeftV1ProcessorImpl<F>
where
    F: Fn(P2VcMeetingParticipantLeftV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2VcMeetingParticipantLeftV1ProcessorImpl { f }
    }
}

/// 视频会议参与者离开事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2VcMeetingParticipantLeftV1Data {
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
    pub participant: LeftMeetingParticipant,
    /// 会议基本信息
    pub meeting: MeetingBasicInfo,
}

/// 离开会议的参与者信息
#[derive(Debug, Serialize, Deserialize)]
pub struct LeftMeetingParticipant {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 参与者角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 加入时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    /// 离开时间 (Unix时间戳，单位：秒)
    pub leave_time: String,
    /// 参与总时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    /// 离开原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_reason: Option<LeaveReason>,
    /// 参与会议期间的统计信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_stats: Option<ParticipantSessionStats>,
    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,
    /// 最终状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_status: Option<ParticipantStatus>,
}

/// 会议基本信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingBasicInfo {
    /// 会议ID
    pub meeting_id: String,
    /// 会议主题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// 会议状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 当前参与人数（离开后的人数）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_participant_count: Option<i32>,
    /// 会议开始时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// 离开原因
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveReason {
    /// 离开类型 (voluntary, kicked_out, network_issue, system_error, device_issue)
    pub leave_type: String,
    /// 详细原因描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_description: Option<String>,
    /// 操作者ID（如果是被踢出）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operated_by: Option<String>,
    /// 是否是意外断开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unexpected: Option<bool>,
    /// 错误代码（如果是系统错误）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

/// 参与者会话统计信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantSessionStats {
    /// 总发言时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_speaking_time: Option<i64>,
    /// 静音状态时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted_duration: Option<i64>,
    /// 视频开启时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_on_duration: Option<i64>,
    /// 屏幕共享时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_share_duration: Option<i64>,
    /// 网络断连次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interruptions: Option<i32>,
    /// 平均网络质量（1-5分，5为最好）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_network_quality: Option<f64>,
    /// 发送的消息数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_sent: Option<i32>,
    /// 举手次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_raise_count: Option<i32>,
    /// 参与互动次数（点赞、鼓掌等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_count: Option<i32>,
}

/// 设备信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// 操作系统
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// 浏览器类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    /// 应用版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// 设备性能指标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_metrics: Option<DevicePerformanceMetrics>,
}

/// 设备性能指标
#[derive(Debug, Serialize, Deserialize)]
pub struct DevicePerformanceMetrics {
    /// CPU使用率峰值（%）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_cpu_usage: Option<f64>,
    /// 内存使用峰值（MB）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_memory_usage: Option<i64>,
    /// 网络带宽峰值（Kbps）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_bandwidth_usage: Option<i32>,
    /// 视频帧率平均值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_video_fps: Option<f64>,
    /// 音频质量评分（1-5）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_quality_score: Option<f64>,
}

/// 参与者状态
#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantStatus {
    /// 最终音频状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_audio_status: Option<String>,
    /// 最终视频状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_video_status: Option<String>,
    /// 是否在共享屏幕
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_screen_sharing: Option<bool>,
    /// 是否举手状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_raised: Option<bool>,
    /// 连接状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
}
