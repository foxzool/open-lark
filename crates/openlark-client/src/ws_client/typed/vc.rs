//! vc 域 typed 事件。
//!
//! 字段权威：Go SDK v3.12.2 `service/vc/v1/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/vc/v1/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `vc.bot.meeting_activity_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotMeetingActivityV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: BotMeetingActivityV1Event,
}

/// `vc.bot.meeting_activity_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotMeetingActivityV1Event {
    /// 会中事件
    #[serde(default)]
    pub meeting_activity_items: Vec<MeetingActivityItem>,
}

/// `vc.bot.meeting_ended_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotMeetingEndedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: BotMeetingEndedV1Event,
}

/// `vc.bot.meeting_ended_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotMeetingEndedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingAgentEventMeeting>,
}

/// `vc.bot.meeting_invited_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotMeetingInvitedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: BotMeetingInvitedV1Event,
}

/// `vc.bot.meeting_invited_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotMeetingInvitedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingAgentEventMeeting>,
    /// 邀请人
    #[serde(default)]
    pub bot: Option<MeetingAgentEventUser>,
    /// 邀请人
    #[serde(default)]
    pub inviter: Option<MeetingAgentEventUser>,
    /// 邀请时间（秒级 unix 时间戳，序列化为字符串避免 JS int64 精度丢失）
    #[serde(default)]
    pub invite_time: Option<String>,
    /// 邀请-入会链路的关联标识，由服务端生成。Agent 收到后必须在调用「加入会议」接口时原样回传。
    #[serde(default)]
    pub call_id: Option<String>,
}

/// `vc.bot.meeting_started_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotMeetingStartedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: BotMeetingStartedV1Event,
}

/// `vc.bot.meeting_started_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotMeetingStartedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingAgentEventMeeting>,
}

/// `vc.meeting.all_meeting_ended_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingAllMeetingEndedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingAllMeetingEndedV1Event,
}

/// `vc.meeting.all_meeting_ended_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingAllMeetingEndedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.meeting.all_meeting_started_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingAllMeetingStartedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingAllMeetingStartedV1Event,
}

/// `vc.meeting.all_meeting_started_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingAllMeetingStartedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.meeting.join_meeting_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingJoinMeetingV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingJoinMeetingV1Event,
}

/// `vc.meeting.join_meeting_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingJoinMeetingV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.meeting.leave_meeting_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingLeaveMeetingV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingLeaveMeetingV1Event,
}

/// `vc.meeting.leave_meeting_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingLeaveMeetingV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
    /// 离开会议原因
    #[serde(default)]
    pub leave_reason: Option<i64>,
    /// 事件操作人
    #[serde(default)]
    pub leave_user: Option<MeetingEventUser>,
}

/// `vc.meeting.meeting_ended_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingEndedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingEndedV1Event,
}

/// `vc.meeting.meeting_ended_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingEndedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.meeting.meeting_started_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingStartedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingStartedV1Event,
}

/// `vc.meeting.meeting_started_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingStartedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.meeting.participant_meeting_ended_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingParticipantMeetingEndedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingParticipantMeetingEndedV1Event,
}

/// `vc.meeting.participant_meeting_ended_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingParticipantMeetingEndedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
    /// 需要推送事件的用户列表
    #[serde(default)]
    pub subscriber_ids: Vec<UserId>,
}

/// `vc.meeting.recording_ended_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingRecordingEndedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingRecordingEndedV1Event,
}

/// `vc.meeting.recording_ended_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingRecordingEndedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.meeting.recording_ready_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingRecordingReadyV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingRecordingReadyV1Event,
}

/// `vc.meeting.recording_ready_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingRecordingReadyV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 会议录制链接
    #[serde(default)]
    pub url: Option<String>,
    /// 录制总时长（单位msec）
    #[serde(default)]
    pub duration: Option<String>,
}

/// `vc.meeting.recording_started_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingRecordingStartedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingRecordingStartedV1Event,
}

/// `vc.meeting.recording_started_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingRecordingStartedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.meeting.share_ended_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingShareEndedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingShareEndedV1Event,
}

/// `vc.meeting.share_ended_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingShareEndedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.meeting.share_started_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingShareStartedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: MeetingShareStartedV1Event,
}

/// `vc.meeting.share_started_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingShareStartedV1Event {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingEventMeeting>,
    /// 事件操作人
    #[serde(default)]
    pub operator: Option<MeetingEventUser>,
}

/// `vc.note.generated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteGeneratedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: NoteGeneratedV1Event,
}

/// `vc.note.generated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteGeneratedV1Event {
    /// 纪要ID
    #[serde(default)]
    pub note_id: Option<String>,
    /// 需要推送事件的用户列表
    #[serde(default)]
    pub subscriber_ids: Vec<UserId>,
}

/// `vc.reserve_config.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReserveConfigUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ReserveConfigUpdatedV1Event,
}

/// `vc.reserve_config.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReserveConfigUpdatedV1Event {
    /// 会议室或层级id
    #[serde(default)]
    pub scope_id: Option<String>,
    /// 1代表层级，2代表会议室
    #[serde(default)]
    pub scope_type: Option<i64>,
    /// 预定审批设置
    #[serde(default)]
    pub approve_config: Option<ApprovalConfigEvent>,
    /// 预定时间设置
    #[serde(default)]
    pub time_config: Option<TimeConfig>,
    /// 预定范围设置
    #[serde(default)]
    pub reserve_scope_config: Option<ReserveScopeConfigEvent>,
}

/// `vc.room.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: RoomCreatedV1Event,
}

/// `vc.room.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomCreatedV1Event {
    /// 会议室详情
    #[serde(default)]
    pub room: Option<RoomEvent>,
}

/// `vc.room.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: RoomDeletedV1Event,
}

/// `vc.room.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomDeletedV1Event {
    /// 会议室详情
    #[serde(default)]
    pub room: Option<RoomEvent>,
}

/// `vc.room.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: RoomUpdatedV1Event,
}

/// `vc.room.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomUpdatedV1Event {
    /// 会议室详情
    #[serde(default)]
    pub room: Option<RoomEvent>,
}

/// `vc.room_level.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomLevelCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: RoomLevelCreatedV1Event,
}

/// `vc.room_level.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomLevelCreatedV1Event {
    /// 层级信息
    #[serde(default)]
    pub room_level: Option<RoomLevel>,
}

/// `vc.room_level.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomLevelDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: RoomLevelDeletedV1Event,
}

/// `vc.room_level.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomLevelDeletedV1Event {
    /// 层级ID
    #[serde(default)]
    pub room_level_id: Option<String>,
    /// 是否删除所有子层级
    #[serde(default)]
    pub delete_child: Option<bool>,
}

/// `vc.room_level.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomLevelUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: RoomLevelUpdatedV1Event,
}

/// `vc.room_level.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomLevelUpdatedV1Event {
    /// 层级信息
    #[serde(default)]
    pub room_level: Option<RoomLevel>,
}

/// 事件嵌套类型（Go `MeetingActivityItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingActivityItem {
    /// 会议数据
    #[serde(default)]
    pub meeting: Option<MeetingAgentEventMeeting>,
    /// 会中活动子类型；取值 participant_joined / participant_left / transcript_received / chat_received / magic_share_started / magic_share_ended
    #[serde(default)]
    pub activity_event_type: Option<String>,
    /// 参会人入会内容（activity_event_type = participant_joined 时填充）
    #[serde(default)]
    pub participant_joined_items: Vec<ParticipantJoinedItem>,
    /// 参会人离会内容（activity_event_type = participant_left 时填充）
    #[serde(default)]
    pub participant_left_items: Vec<ParticipantLeftItem>,
    /// 字幕内容（activity_event_type = transcript_received 时填充）
    #[serde(default)]
    pub transcript_received_items: Vec<TranscriptItem>,
    /// 聊天消息内容（activity_event_type = chat_received 时填充）
    #[serde(default)]
    pub chat_received_items: Vec<ChatMessageItem>,
    /// 妙享开始内容（activity_event_type = magic_share_started 时填充）
    #[serde(default)]
    pub magic_share_started_items: Vec<MagicShareStartedItem>,
    /// 妙享结束内容（activity_event_type = magic_share_ended 时填充）
    #[serde(default)]
    pub magic_share_ended_items: Vec<MagicShareEndedItem>,
    /// 文档上下文变化内容（activity_event_type = document_context_changed 时返回），单条记录包含评论聚焦、章节定位或元素预览中的一种变化
    #[serde(default)]
    pub document_context_changed_items: Vec<DocumentContextChangedItem>,
    /// 倒计时状态变化内容（activity_event_type = countdown_changed 时填充）
    #[serde(default)]
    pub countdown_items: Vec<CountdownItem>,
}

/// 事件嵌套类型（Go `MeetingAgentEventMeeting`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingAgentEventMeeting {
    /// 会议ID
    #[serde(default)]
    pub id: Option<String>,
    /// 会议主题
    #[serde(default)]
    pub topic: Option<String>,
    /// 9位会议号
    #[serde(default)]
    pub meeting_no: Option<String>,
    /// 会议开始时间（unix时间，单位sec）
    #[serde(default)]
    pub start_time: Option<String>,
    /// 会议结束时间（unix时间，单位sec）
    #[serde(default)]
    pub end_time: Option<String>,
    /// 邀请人
    #[serde(default)]
    pub host_user: Option<MeetingAgentEventUser>,
}

/// 事件嵌套类型（Go `MeetingAgentEventUser`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingAgentEventUser {
    /// 用户 ID
    #[serde(default)]
    pub id: Option<UserId>,
    /// 用户类型
    #[serde(default)]
    pub user_type: Option<i64>,
    /// 用户角色
    #[serde(default)]
    pub user_role: Option<i64>,
    /// 用户名称
    #[serde(default)]
    pub user_name: Option<String>,
}

/// 事件嵌套类型（Go `MeetingEventMeeting`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingEventMeeting {
    /// 会议ID（视频会议的唯一标识，视频会议开始后才会产生）
    #[serde(default)]
    pub id: Option<String>,
    /// 会议主题
    #[serde(default)]
    pub topic: Option<String>,
    /// 9位会议号（飞书用户可通过输入9位会议号快捷入会）
    #[serde(default)]
    pub meeting_no: Option<String>,
    /// 会议创建源
    #[serde(default)]
    pub meeting_source: Option<i64>,
    /// 会议开始时间（unix时间，单位：秒）
    #[serde(default)]
    pub start_time: Option<String>,
    /// 会议结束时间（unix时间，单位：秒）
    #[serde(default)]
    pub end_time: Option<String>,
    /// 事件操作人
    #[serde(default)]
    pub host_user: Option<MeetingEventUser>,
    /// 事件操作人
    #[serde(default)]
    pub owner: Option<MeetingEventUser>,
    /// 日程实体的唯一标志
    #[serde(default)]
    pub calendar_event_id: Option<String>,
    /// 会议子类型
    #[serde(default)]
    pub meeting_sub_type: Option<i64>,
    /// 会议安全设置
    #[serde(default)]
    pub security_setting: Option<MeetingSecuritySetting>,
    /// 研讨会相关设置
    #[serde(default)]
    pub webinar_setting: Option<MeetingWebinarSetting>,
}

/// 事件嵌套类型（Go `MeetingEventUser`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingEventUser {
    /// 用户ID
    #[serde(default)]
    pub id: Option<UserId>,
    /// 用户会中角色
    #[serde(default)]
    pub user_role: Option<i64>,
    /// 用户类型
    #[serde(default)]
    pub user_type: Option<i64>,
}

/// 事件嵌套类型（Go `UserId`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserId {
    /// 对应 Go json tag `user_id`。
    #[serde(default)]
    pub user_id: Option<String>,
    /// 对应 Go json tag `open_id`。
    #[serde(default)]
    pub open_id: Option<String>,
    /// 对应 Go json tag `union_id`。
    #[serde(default)]
    pub union_id: Option<String>,
}

/// 事件嵌套类型（Go `ApprovalConfigEvent`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApprovalConfigEvent {
    /// 预定审批开关，0关闭，1打开
    #[serde(default)]
    pub approval_switch: Option<i64>,
    /// 预定审批条件，0所有预定需要审批，1满足条件需审批
    #[serde(default)]
    pub approval_condition: Option<i64>,
    /// 超过 meeting_duration小时需要审批
    #[serde(default)]
    pub meeting_duration: Option<f64>,
    /// 审批人列表
    #[serde(default)]
    pub approvers: Vec<SubscribeUserEvent>,
}

/// 事件嵌套类型（Go `TimeConfig`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeConfig {
    /// 是否覆盖子层级及会议室
    #[serde(default)]
    pub if_cover_child_scope: Option<bool>,
    /// 预定时间开关：0 代表关闭，1 代表开启
    #[serde(default)]
    pub time_switch: Option<i64>,
    /// 最早可提前 ; days_in_advance 预定会议室（单位：天，取值范围\[1-730\]）;<b>说明</b>：不填写时，默认更新为 365
    #[serde(default)]
    pub days_in_advance: Option<i64>,
    /// 开放当天可于 ; opening_hour 开始预定（单位：秒，取值范围\[0,86400\]）;<b>说明</b>：;1. 不填写时默认更新为 ; 28800 ;2. 如果填写的值不是 60 ; 的倍数，则自动会更新为离其最近的 60 整数倍的值。
    #[serde(default)]
    pub opening_hour: Option<String>,
    /// 每日可预定时间范围的开始时间（单位：秒，取值范围\[0,86400\]）;<b>说明</b>：;1. 不填写时，默认更新为 0 ，此时填写的 end_time 不得小于 30。;2. 当 start_time 与; end_time 均填写时，; end_time 至少超过 ; start_time 30 。;3. 如果填写的值不是 60 的倍数，则自动会更新为离其最近的 60 整数倍的值。
    #[serde(default)]
    pub start_time: Option<String>,
    /// 每日可预定时间范围结束时间（单位：秒，取值范围\[0,86400\]）;<b>说明</b>：;1. 不填写时，默认更新为 86400 ，此时填写的; start_time 不得大于等于 86370 。;2. 当 start_time 与; end_time 均填写时，; end_time 至少要超过; start_time 30。;3. 如果填写的值不是 60 的倍数，则自动会更新为离其最近的 60 整数倍的值。
    #[serde(default)]
    pub end_time: Option<String>,
    /// 单次会议室可预定时长上限（单位：小时，取值范围\[1,99\]）;<b>说明</b>：不填写时默认更新为 2
    #[serde(default)]
    pub max_duration: Option<i64>,
}

/// 事件嵌套类型（Go `ReserveScopeConfigEvent`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReserveScopeConfigEvent {
    /// 可预定成员范围，0部分成员，1全部成员
    #[serde(default)]
    pub allow_all_users: Option<i64>,
    /// 可预定成员列表
    #[serde(default)]
    pub allow_users: Vec<SubscribeUserEvent>,
    /// 可预定部门列表
    #[serde(default)]
    pub allow_depts: Vec<SubscribeDepartment>,
}

/// 事件嵌套类型（Go `RoomEvent`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoomEvent {
    /// 会议室ID
    #[serde(default)]
    pub room_id: Option<String>,
    /// 会议室名称
    #[serde(default)]
    pub name: Option<String>,
    /// 会议室能容纳的人数
    #[serde(default)]
    pub capacity: Option<i64>,
    /// 会议室的相关描述
    #[serde(default)]
    pub description: Option<String>,
    /// 会议室的展示ID
    #[serde(default)]
    pub display_id: Option<String>,
    /// 自定义的会议室ID
    #[serde(default)]
    pub custom_room_id: Option<String>,
    /// 层级ID
    #[serde(default)]
    pub room_level_id: Option<String>,
    /// 层级路径
    #[serde(default)]
    pub path: Vec<String>,
    /// 会议室状态
    #[serde(default)]
    pub room_status: Option<RoomStatusEvent>,
    /// 设施信息列表
    #[serde(default)]
    pub device: Vec<Device>,
}

/// 事件嵌套类型（Go `RoomLevel`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoomLevel {
    /// 层级ID
    #[serde(default)]
    pub room_level_id: Option<String>,
    /// 层级名称
    #[serde(default)]
    pub name: Option<String>,
    /// 父层级ID
    #[serde(default)]
    pub parent_id: Option<String>,
    /// 层级路径
    #[serde(default)]
    pub path: Vec<String>,
    /// 是否有子层级
    #[serde(default)]
    pub has_child: Option<bool>,
    /// 自定义层级ID
    #[serde(default)]
    pub custom_group_id: Option<String>,
}

/// 事件嵌套类型（Go `ParticipantJoinedItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantJoinedItem {
    /// 邀请人
    #[serde(default)]
    pub participant: Option<MeetingAgentEventUser>,
    /// 入会时间（毫秒级时间戳）
    #[serde(default)]
    pub join_time: Option<String>,
}

/// 事件嵌套类型（Go `ParticipantLeftItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantLeftItem {
    /// 邀请人
    #[serde(default)]
    pub participant: Option<MeetingAgentEventUser>,
    /// 离会原因
    #[serde(default)]
    pub leave_reason: Option<i64>,
    /// 离会时间（毫秒级时间戳）
    #[serde(default)]
    pub leave_time: Option<String>,
}

/// 事件嵌套类型（Go `TranscriptItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TranscriptItem {
    /// 邀请人
    #[serde(default)]
    pub speaker: Option<MeetingAgentEventUser>,
    /// 字幕文本
    #[serde(default)]
    pub text: Option<String>,
    /// 语言类型
    #[serde(default)]
    pub language: Option<String>,
    /// 句子开始时间（毫秒级时间戳）
    #[serde(default)]
    pub start_time_ms: Option<String>,
    /// 句子结束时间（毫秒级时间戳）
    #[serde(default)]
    pub end_time_ms: Option<String>,
    /// 句子 ID（用于去重和排序）
    #[serde(default)]
    pub sentence_id: Option<String>,
}

/// 事件嵌套类型（Go `ChatMessageItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatMessageItem {
    /// 邀请人
    #[serde(default)]
    pub operator: Option<MeetingAgentEventUser>,
    /// 消息 ID
    #[serde(default)]
    pub message_id: Option<String>,
    /// 消息类型
    #[serde(default)]
    pub message_type: Option<i64>,
    /// 消息内容
    #[serde(default)]
    pub content: Option<String>,
    /// 发送时间（毫秒级时间戳）
    #[serde(default)]
    pub send_time: Option<String>,
}

/// 事件嵌套类型（Go `MagicShareStartedItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MagicShareStartedItem {
    /// 邀请人
    #[serde(default)]
    pub operator: Option<MeetingAgentEventUser>,
    /// 共享会话 ID
    #[serde(default)]
    pub share_id: Option<String>,
    /// 共享文档信息
    #[serde(default)]
    pub share_doc: Option<ShareDoc>,
    /// 妙享开始时间（毫秒级时间戳）
    #[serde(default)]
    pub time: Option<String>,
    /// 共享开始原因。share_started 表示真实共享开始；share_detected 表示开启 Agent 入会能力时发现已有共享。字段缺失按 share_started 处理。
    #[serde(default)]
    pub start_reason: Option<String>,
}

/// 事件嵌套类型（Go `MagicShareEndedItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MagicShareEndedItem {
    /// 邀请人
    #[serde(default)]
    pub operator: Option<MeetingAgentEventUser>,
    /// 共享会话 ID
    #[serde(default)]
    pub share_id: Option<String>,
    /// 妙享结束时间（毫秒级时间戳）
    #[serde(default)]
    pub time: Option<String>,
}

/// 事件嵌套类型（Go `DocumentContextChangedItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentContextChangedItem {
    /// 邀请人
    #[serde(default)]
    pub operator: Option<MeetingAgentEventUser>,
    /// 共享会话 ID
    #[serde(default)]
    pub share_id: Option<String>,
    /// 当前共享文档信息
    #[serde(default)]
    pub share_doc: Option<ShareDoc>,
    /// 事件发生时间，十进制毫秒级时间戳字符串
    #[serde(default)]
    pub time: Option<String>,
    /// 评论聚焦变化，仅评论聚焦或取消聚焦时返回
    #[serde(default)]
    pub comment_focus: Option<CommentFocus>,
    /// 章节位置变化，仅章节定位变化时返回
    #[serde(default)]
    pub section_location: Option<SectionLocation>,
    /// 元素预览变化，仅图片或白板预览变化时返回
    #[serde(default)]
    pub element_preview: Option<ElementPreview>,
}

/// 事件嵌套类型（Go `CountdownItem`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CountdownItem {
    /// 倒计时动作；取值 SET / PROLONG / END_IN_ADVANCE / CLOSE / ENDED / REMIND
    #[serde(default)]
    pub action: Option<String>,
    /// 邀请人
    #[serde(default)]
    pub operator: Option<MeetingAgentEventUser>,
    /// 倒计时结束时间（毫秒级时间戳）
    #[serde(default)]
    pub end_time: Option<String>,
    /// 倒计时事件发生时间（毫秒级时间戳）
    #[serde(default)]
    pub event_time: Option<String>,
    /// 倒计时结束时是否播放提示音
    #[serde(default)]
    pub need_play_audio_at_end: Option<bool>,
    /// 倒计时结束前提醒时间列表，单位为秒
    #[serde(default)]
    pub reminders_before_end_in_second: Vec<i64>,
    /// 倒计时序列 ID
    #[serde(default)]
    pub seq_id: Option<String>,
    /// 倒计时设置时间（毫秒级时间戳）
    #[serde(default)]
    pub countdown_set_time: Option<String>,
    /// 距离倒计时结束的剩余分钟数；仅 action 为 REMIND 时填充
    #[serde(default)]
    pub remain_minutes: Option<i64>,
}

/// 事件嵌套类型（Go `MeetingSecuritySetting`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingSecuritySetting {
    /// 安全级别
    #[serde(default)]
    pub security_level: Option<i64>,
    /// 允许入会的群组ID列表
    #[serde(default)]
    pub group_ids: Vec<String>,
    /// 允许入会的用户ID列表
    #[serde(default)]
    pub user_ids: Vec<UserId>,
    /// 允许入会的会议室ID列表
    #[serde(default)]
    pub room_ids: Vec<String>,
    /// 是否设置了仅指定联系人和群组可参会
    #[serde(default)]
    pub has_set_security_contacts_and_group: Option<bool>,
}

/// 事件嵌套类型（Go `MeetingWebinarSetting`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingWebinarSetting {
    /// 网络研讨会类型
    #[serde(default)]
    pub webinar_type: Option<i64>,
}

/// 事件嵌套类型（Go `SubscribeUserEvent`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeUserEvent {
    /// 预定人/审批人id
    #[serde(default)]
    pub user_id: Option<UserId>,
}

/// 事件嵌套类型（Go `SubscribeDepartment`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeDepartment {
    /// 预定管理部门ID，使用open_department_id
    #[serde(default)]
    pub department_id: Option<String>,
    /// 预定部门名称
    #[serde(default)]
    pub department_name: Option<String>,
}

/// 事件嵌套类型（Go `RoomStatusEvent`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoomStatusEvent {
    /// 是否启用会议室
    #[serde(default)]
    pub status: Option<bool>,
    /// 会议室未来状态为启用或禁用
    #[serde(default)]
    pub schedule_status: Option<bool>,
    /// 禁用开始时间（unix时间，单位sec）
    #[serde(default)]
    pub disable_start_time: Option<String>,
    /// 禁用结束时间（unix时间，单位sec，数值0表示永久禁用）
    #[serde(default)]
    pub disable_end_time: Option<String>,
    /// 禁用原因
    #[serde(default)]
    pub disable_reason: Option<String>,
    /// 联系人列表
    #[serde(default)]
    pub contact_ids: Vec<UserId>,
    /// 是否在禁用时发送通知给预定了该会议室的员工
    #[serde(default)]
    pub disable_notice: Option<bool>,
    /// 是否在恢复启用时发送通知给预定了该会议室的员工
    #[serde(default)]
    pub resume_notice: Option<bool>,
}

/// 事件嵌套类型（Go `Device`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Device {
    /// 设施名称
    #[serde(default)]
    pub name: Option<String>,
}

/// 事件嵌套类型（Go `ShareDoc`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareDoc {
    /// 文档 URL
    #[serde(default)]
    pub url: Option<String>,
    /// 文档标题
    #[serde(default)]
    pub title: Option<String>,
}

/// 事件嵌套类型（Go `CommentFocus`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommentFocus {
    /// 评论 ID
    #[serde(default)]
    pub comment_id: Option<String>,
    /// 是否聚焦该评论，true 表示聚焦，false 表示取消聚焦
    #[serde(default)]
    pub focused: Option<bool>,
}

/// 事件嵌套类型（Go `SectionLocation`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionLocation {
    /// 当前章节标题
    #[serde(default)]
    pub title: Option<String>,
    /// 当前章节层级
    #[serde(default)]
    pub level: Option<i64>,
    /// 父级章节标题列表
    #[serde(default)]
    pub parent_titles: Vec<String>,
}

/// 事件嵌套类型（Go `ElementPreview`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElementPreview {
    /// 元素预览动作
    #[serde(default)]
    pub action: Option<String>,
    /// 预览元素类型
    #[serde(default)]
    pub element_type: Option<String>,
    /// 预览元素 Token
    #[serde(default)]
    pub element_token: Option<String>,
    /// 元素所在的文档 Block ID
    #[serde(default)]
    pub block_id: Option<String>,
}

typed_events! {
    /// `vc.bot.meeting_activity_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.bot.meeting_activity_v1" | VC_BOT_MEETING_ACTIVITY_V1 | BotMeetingActivityV1 | BotMeetingActivityV1Handler | register_vc_bot_meeting_activity_v1;
    /// `vc.bot.meeting_ended_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.bot.meeting_ended_v1" | VC_BOT_MEETING_ENDED_V1 | BotMeetingEndedV1 | BotMeetingEndedV1Handler | register_vc_bot_meeting_ended_v1;
    /// `vc.bot.meeting_invited_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.bot.meeting_invited_v1" | VC_BOT_MEETING_INVITED_V1 | BotMeetingInvitedV1 | BotMeetingInvitedV1Handler | register_vc_bot_meeting_invited_v1;
    /// `vc.bot.meeting_started_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.bot.meeting_started_v1" | VC_BOT_MEETING_STARTED_V1 | BotMeetingStartedV1 | BotMeetingStartedV1Handler | register_vc_bot_meeting_started_v1;
    /// `vc.meeting.all_meeting_ended_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.all_meeting_ended_v1" | VC_MEETING_ALL_MEETING_ENDED_V1 | MeetingAllMeetingEndedV1 | MeetingAllMeetingEndedV1Handler | register_vc_meeting_all_meeting_ended_v1;
    /// `vc.meeting.all_meeting_started_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.all_meeting_started_v1" | VC_MEETING_ALL_MEETING_STARTED_V1 | MeetingAllMeetingStartedV1 | MeetingAllMeetingStartedV1Handler | register_vc_meeting_all_meeting_started_v1;
    /// `vc.meeting.join_meeting_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.join_meeting_v1" | VC_MEETING_JOIN_MEETING_V1 | MeetingJoinMeetingV1 | MeetingJoinMeetingV1Handler | register_vc_meeting_join_meeting_v1;
    /// `vc.meeting.leave_meeting_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.leave_meeting_v1" | VC_MEETING_LEAVE_MEETING_V1 | MeetingLeaveMeetingV1 | MeetingLeaveMeetingV1Handler | register_vc_meeting_leave_meeting_v1;
    /// `vc.meeting.meeting_ended_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.meeting_ended_v1" | VC_MEETING_MEETING_ENDED_V1 | MeetingEndedV1 | MeetingEndedV1Handler | register_vc_meeting_meeting_ended_v1;
    /// `vc.meeting.meeting_started_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.meeting_started_v1" | VC_MEETING_MEETING_STARTED_V1 | MeetingStartedV1 | MeetingStartedV1Handler | register_vc_meeting_meeting_started_v1;
    /// `vc.meeting.participant_meeting_ended_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.participant_meeting_ended_v1" | VC_MEETING_PARTICIPANT_MEETING_ENDED_V1 | MeetingParticipantMeetingEndedV1 | MeetingParticipantMeetingEndedV1Handler | register_vc_meeting_participant_meeting_ended_v1;
    /// `vc.meeting.recording_ended_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.recording_ended_v1" | VC_MEETING_RECORDING_ENDED_V1 | MeetingRecordingEndedV1 | MeetingRecordingEndedV1Handler | register_vc_meeting_recording_ended_v1;
    /// `vc.meeting.recording_ready_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.recording_ready_v1" | VC_MEETING_RECORDING_READY_V1 | MeetingRecordingReadyV1 | MeetingRecordingReadyV1Handler | register_vc_meeting_recording_ready_v1;
    /// `vc.meeting.recording_started_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.recording_started_v1" | VC_MEETING_RECORDING_STARTED_V1 | MeetingRecordingStartedV1 | MeetingRecordingStartedV1Handler | register_vc_meeting_recording_started_v1;
    /// `vc.meeting.share_ended_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.share_ended_v1" | VC_MEETING_SHARE_ENDED_V1 | MeetingShareEndedV1 | MeetingShareEndedV1Handler | register_vc_meeting_share_ended_v1;
    /// `vc.meeting.share_started_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.meeting.share_started_v1" | VC_MEETING_SHARE_STARTED_V1 | MeetingShareStartedV1 | MeetingShareStartedV1Handler | register_vc_meeting_share_started_v1;
    /// `vc.note.generated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.note.generated_v1" | VC_NOTE_GENERATED_V1 | NoteGeneratedV1 | NoteGeneratedV1Handler | register_vc_note_generated_v1;
    /// `vc.reserve_config.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.reserve_config.updated_v1" | VC_RESERVE_CONFIG_UPDATED_V1 | ReserveConfigUpdatedV1 | ReserveConfigUpdatedV1Handler | register_vc_reserve_config_updated_v1;
    /// `vc.room.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.room.created_v1" | VC_ROOM_CREATED_V1 | RoomCreatedV1 | RoomCreatedV1Handler | register_vc_room_created_v1;
    /// `vc.room.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.room.deleted_v1" | VC_ROOM_DELETED_V1 | RoomDeletedV1 | RoomDeletedV1Handler | register_vc_room_deleted_v1;
    /// `vc.room.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.room.updated_v1" | VC_ROOM_UPDATED_V1 | RoomUpdatedV1 | RoomUpdatedV1Handler | register_vc_room_updated_v1;
    /// `vc.room_level.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.room_level.created_v1" | VC_ROOM_LEVEL_CREATED_V1 | RoomLevelCreatedV1 | RoomLevelCreatedV1Handler | register_vc_room_level_created_v1;
    /// `vc.room_level.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.room_level.deleted_v1" | VC_ROOM_LEVEL_DELETED_V1 | RoomLevelDeletedV1 | RoomLevelDeletedV1Handler | register_vc_room_level_deleted_v1;
    /// `vc.room_level.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "vc.room_level.updated_v1" | VC_ROOM_LEVEL_UPDATED_V1 | RoomLevelUpdatedV1 | RoomLevelUpdatedV1Handler | register_vc_room_level_updated_v1;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const VC_BOT_MEETING_ACTIVITY_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.bot.meeting_activity_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting_activity_items": [
      {
        "meeting": {
          "id": "id_str",
          "topic": "topic_str",
          "meeting_no": "meeting_no_str",
          "start_time": "start_time_str",
          "end_time": "end_time_str",
          "host_user": {
            "id": {
              "user_id": "user_id_str",
              "open_id": "open_id_str",
              "union_id": "union_id_str"
            },
            "user_type": 42,
            "user_role": 42,
            "user_name": "user_name_str"
          }
        },
        "activity_event_type": "activity_event_type_str",
        "participant_joined_items": [
          {
            "participant": {
              "id": {
                "user_id": "user_id_str",
                "open_id": "open_id_str",
                "union_id": "union_id_str"
              },
              "user_type": 42,
              "user_role": 42,
              "user_name": "user_name_str"
            },
            "join_time": "join_time_str"
          }
        ],
        "participant_left_items": [
          {
            "participant": {
              "id": {
                "user_id": "user_id_str",
                "open_id": "open_id_str",
                "union_id": "union_id_str"
              },
              "user_type": 42,
              "user_role": 42,
              "user_name": "user_name_str"
            },
            "leave_reason": 42,
            "leave_time": "leave_time_str"
          }
        ],
        "transcript_received_items": [
          {
            "speaker": {
              "id": {
                "user_id": "user_id_str",
                "open_id": "open_id_str",
                "union_id": "union_id_str"
              },
              "user_type": 42,
              "user_role": 42,
              "user_name": "user_name_str"
            },
            "text": "text_str",
            "language": "language_str",
            "start_time_ms": "start_time_ms_str",
            "end_time_ms": "end_time_ms_str",
            "sentence_id": "sentence_id_str"
          }
        ],
        "chat_received_items": [
          {
            "operator": {
              "id": {
                "user_id": "user_id_str",
                "open_id": "open_id_str",
                "union_id": "union_id_str"
              },
              "user_type": 42,
              "user_role": 42,
              "user_name": "user_name_str"
            },
            "message_id": "message_id_str",
            "message_type": 42,
            "content": "content_str",
            "send_time": "send_time_str"
          }
        ],
        "magic_share_started_items": [
          {
            "operator": {
              "id": {
                "user_id": "user_id_str",
                "open_id": "open_id_str",
                "union_id": "union_id_str"
              },
              "user_type": 42,
              "user_role": 42,
              "user_name": "user_name_str"
            },
            "share_id": "share_id_str",
            "share_doc": {
              "url": "url_str",
              "title": "title_str"
            },
            "time": "time_str",
            "start_reason": "start_reason_str"
          }
        ],
        "magic_share_ended_items": [
          {
            "operator": {
              "id": {
                "user_id": "user_id_str",
                "open_id": "open_id_str",
                "union_id": "union_id_str"
              },
              "user_type": 42,
              "user_role": 42,
              "user_name": "user_name_str"
            },
            "share_id": "share_id_str",
            "time": "time_str"
          }
        ],
        "document_context_changed_items": [
          {
            "operator": {
              "id": {
                "user_id": "user_id_str",
                "open_id": "open_id_str",
                "union_id": "union_id_str"
              },
              "user_type": 42,
              "user_role": 42,
              "user_name": "user_name_str"
            },
            "share_id": "share_id_str",
            "share_doc": {
              "url": "url_str",
              "title": "title_str"
            },
            "time": "time_str",
            "comment_focus": {
              "comment_id": "comment_id_str",
              "focused": true
            },
            "section_location": {
              "title": "title_str",
              "level": 42,
              "parent_titles": [
                "parent_titles_str"
              ]
            },
            "element_preview": {
              "action": "action_str",
              "element_type": "element_type_str",
              "element_token": "element_token_str",
              "block_id": "block_id_str"
            }
          }
        ],
        "countdown_items": [
          {
            "action": "action_str",
            "operator": {
              "id": {
                "user_id": "user_id_str",
                "open_id": "open_id_str",
                "union_id": "union_id_str"
              },
              "user_type": 42,
              "user_role": 42,
              "user_name": "user_name_str"
            },
            "end_time": "end_time_str",
            "event_time": "event_time_str",
            "need_play_audio_at_end": true,
            "reminders_before_end_in_second": [
              42
            ],
            "seq_id": "seq_id_str",
            "countdown_set_time": "countdown_set_time_str",
            "remain_minutes": 42
          }
        ]
      }
    ]
  }
}"#;

    const VC_BOT_MEETING_ENDED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.bot.meeting_ended_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_type": 42,
        "user_role": 42,
        "user_name": "user_name_str"
      }
    }
  }
}"#;

    const VC_BOT_MEETING_INVITED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.bot.meeting_invited_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_type": 42,
        "user_role": 42,
        "user_name": "user_name_str"
      }
    },
    "bot": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_type": 42,
      "user_role": 42,
      "user_name": "user_name_str"
    },
    "inviter": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_type": 42,
      "user_role": 42,
      "user_name": "user_name_str"
    },
    "invite_time": "invite_time_str",
    "call_id": "call_id_str"
  }
}"#;

    const VC_BOT_MEETING_STARTED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.bot.meeting_started_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_type": 42,
        "user_role": 42,
        "user_name": "user_name_str"
      }
    }
  }
}"#;

    const VC_MEETING_ALL_MEETING_ENDED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.all_meeting_ended_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_ALL_MEETING_STARTED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.all_meeting_started_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_JOIN_MEETING_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.join_meeting_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_LEAVE_MEETING_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.leave_meeting_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    },
    "leave_reason": 42,
    "leave_user": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_MEETING_ENDED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.meeting_ended_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_MEETING_STARTED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.meeting_started_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_PARTICIPANT_MEETING_ENDED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.participant_meeting_ended_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    },
    "subscriber_ids": [
      {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      }
    ]
  }
}"#;

    const VC_MEETING_RECORDING_ENDED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.recording_ended_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_RECORDING_READY_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.recording_ready_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "url": "url_str",
    "duration": "duration_str"
  }
}"#;

    const VC_MEETING_RECORDING_STARTED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.recording_started_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_SHARE_ENDED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.share_ended_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_MEETING_SHARE_STARTED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.meeting.share_started_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "meeting": {
      "id": "id_str",
      "topic": "topic_str",
      "meeting_no": "meeting_no_str",
      "meeting_source": 42,
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "host_user": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "owner": {
        "id": {
          "user_id": "user_id_str",
          "open_id": "open_id_str",
          "union_id": "union_id_str"
        },
        "user_role": 42,
        "user_type": 42
      },
      "calendar_event_id": "calendar_event_id_str",
      "meeting_sub_type": 42,
      "security_setting": {
        "security_level": 42,
        "group_ids": [
          "group_ids_str"
        ],
        "user_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "room_ids": [
          "room_ids_str"
        ],
        "has_set_security_contacts_and_group": true
      },
      "webinar_setting": {
        "webinar_type": 42
      }
    },
    "operator": {
      "id": {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      },
      "user_role": 42,
      "user_type": 42
    }
  }
}"#;

    const VC_NOTE_GENERATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.note.generated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "note_id": "note_id_str",
    "subscriber_ids": [
      {
        "user_id": "user_id_str",
        "open_id": "open_id_str",
        "union_id": "union_id_str"
      }
    ]
  }
}"#;

    const VC_RESERVE_CONFIG_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.reserve_config.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "scope_id": "scope_id_str",
    "scope_type": 42,
    "approve_config": {
      "approval_switch": 42,
      "approval_condition": 42,
      "meeting_duration": 1.5,
      "approvers": [
        {
          "user_id": {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        }
      ]
    },
    "time_config": {
      "if_cover_child_scope": true,
      "time_switch": 42,
      "days_in_advance": 42,
      "opening_hour": "opening_hour_str",
      "start_time": "start_time_str",
      "end_time": "end_time_str",
      "max_duration": 42
    },
    "reserve_scope_config": {
      "allow_all_users": 42,
      "allow_users": [
        {
          "user_id": {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        }
      ],
      "allow_depts": [
        {
          "department_id": "department_id_str",
          "department_name": "department_name_str"
        }
      ]
    }
  }
}"#;

    const VC_ROOM_CREATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.room.created_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "room": {
      "room_id": "room_id_str",
      "name": "name_str",
      "capacity": 42,
      "description": "description_str",
      "display_id": "display_id_str",
      "custom_room_id": "custom_room_id_str",
      "room_level_id": "room_level_id_str",
      "path": [
        "path_str"
      ],
      "room_status": {
        "status": true,
        "schedule_status": true,
        "disable_start_time": "disable_start_time_str",
        "disable_end_time": "disable_end_time_str",
        "disable_reason": "disable_reason_str",
        "contact_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "disable_notice": true,
        "resume_notice": true
      },
      "device": [
        {
          "name": "name_str"
        }
      ]
    }
  }
}"#;

    const VC_ROOM_DELETED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.room.deleted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "room": {
      "room_id": "room_id_str",
      "name": "name_str",
      "capacity": 42,
      "description": "description_str",
      "display_id": "display_id_str",
      "custom_room_id": "custom_room_id_str",
      "room_level_id": "room_level_id_str",
      "path": [
        "path_str"
      ],
      "room_status": {
        "status": true,
        "schedule_status": true,
        "disable_start_time": "disable_start_time_str",
        "disable_end_time": "disable_end_time_str",
        "disable_reason": "disable_reason_str",
        "contact_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "disable_notice": true,
        "resume_notice": true
      },
      "device": [
        {
          "name": "name_str"
        }
      ]
    }
  }
}"#;

    const VC_ROOM_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.room.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "room": {
      "room_id": "room_id_str",
      "name": "name_str",
      "capacity": 42,
      "description": "description_str",
      "display_id": "display_id_str",
      "custom_room_id": "custom_room_id_str",
      "room_level_id": "room_level_id_str",
      "path": [
        "path_str"
      ],
      "room_status": {
        "status": true,
        "schedule_status": true,
        "disable_start_time": "disable_start_time_str",
        "disable_end_time": "disable_end_time_str",
        "disable_reason": "disable_reason_str",
        "contact_ids": [
          {
            "user_id": "user_id_str",
            "open_id": "open_id_str",
            "union_id": "union_id_str"
          }
        ],
        "disable_notice": true,
        "resume_notice": true
      },
      "device": [
        {
          "name": "name_str"
        }
      ]
    }
  }
}"#;

    const VC_ROOM_LEVEL_CREATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.room_level.created_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "room_level": {
      "room_level_id": "room_level_id_str",
      "name": "name_str",
      "parent_id": "parent_id_str",
      "path": [
        "path_str"
      ],
      "has_child": true,
      "custom_group_id": "custom_group_id_str"
    }
  }
}"#;

    const VC_ROOM_LEVEL_DELETED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.room_level.deleted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "room_level_id": "room_level_id_str",
    "delete_child": true
  }
}"#;

    const VC_ROOM_LEVEL_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "vc.room_level.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "room_level": {
      "room_level_id": "room_level_id_str",
      "name": "name_str",
      "parent_id": "parent_id_str",
      "path": [
        "path_str"
      ],
      "has_child": true,
      "custom_group_id": "custom_group_id_str"
    }
  }
}"#;

    #[test]
    fn typed_vc_bot_meeting_activity_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_bot_meeting_activity_v1(move |event: BotMeetingActivityV1| {
                assert_eq!(event.header.event_type, "vc.bot.meeting_activity_v1");
                assert_eq!(event.event.meeting_activity_items.len(), 1);
                assert_eq!(
                    event.event.meeting_activity_items[0]
                        .activity_event_type
                        .as_deref(),
                    Some("activity_event_type_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_BOT_MEETING_ACTIVITY_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_bot_meeting_ended_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_bot_meeting_ended_v1(move |event: BotMeetingEndedV1| {
                assert_eq!(event.header.event_type, "vc.bot.meeting_ended_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_BOT_MEETING_ENDED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_bot_meeting_invited_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_bot_meeting_invited_v1(move |event: BotMeetingInvitedV1| {
                assert_eq!(event.header.event_type, "vc.bot.meeting_invited_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                assert_eq!(event.event.invite_time.as_deref(), Some("invite_time_str"));
                assert_eq!(event.event.call_id.as_deref(), Some("call_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_BOT_MEETING_INVITED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_bot_meeting_started_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_bot_meeting_started_v1(move |event: BotMeetingStartedV1| {
                assert_eq!(event.header.event_type, "vc.bot.meeting_started_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_BOT_MEETING_STARTED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_all_meeting_ended_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_all_meeting_ended_v1(move |event: MeetingAllMeetingEndedV1| {
                assert_eq!(event.header.event_type, "vc.meeting.all_meeting_ended_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_ALL_MEETING_ENDED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_all_meeting_started_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_all_meeting_started_v1(move |event: MeetingAllMeetingStartedV1| {
                assert_eq!(event.header.event_type, "vc.meeting.all_meeting_started_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_ALL_MEETING_STARTED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_join_meeting_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_join_meeting_v1(move |event: MeetingJoinMeetingV1| {
                assert_eq!(event.header.event_type, "vc.meeting.join_meeting_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_JOIN_MEETING_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_leave_meeting_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_leave_meeting_v1(move |event: MeetingLeaveMeetingV1| {
                assert_eq!(event.header.event_type, "vc.meeting.leave_meeting_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                assert_eq!(event.event.leave_reason, Some(42));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_LEAVE_MEETING_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_meeting_ended_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_meeting_ended_v1(move |event: MeetingEndedV1| {
                assert_eq!(event.header.event_type, "vc.meeting.meeting_ended_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_MEETING_ENDED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_meeting_started_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_meeting_started_v1(move |event: MeetingStartedV1| {
                assert_eq!(event.header.event_type, "vc.meeting.meeting_started_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_MEETING_STARTED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_participant_meeting_ended_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_participant_meeting_ended_v1(
                move |event: MeetingParticipantMeetingEndedV1| {
                    assert_eq!(
                        event.header.event_type,
                        "vc.meeting.participant_meeting_ended_v1"
                    );
                    let meeting = event.event.meeting.as_ref().expect("meeting");
                    assert_eq!(meeting.id.as_deref(), Some("id_str"));
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_PARTICIPANT_MEETING_ENDED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_recording_ended_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_recording_ended_v1(move |event: MeetingRecordingEndedV1| {
                assert_eq!(event.header.event_type, "vc.meeting.recording_ended_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_RECORDING_ENDED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_recording_ready_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_recording_ready_v1(move |event: MeetingRecordingReadyV1| {
                assert_eq!(event.header.event_type, "vc.meeting.recording_ready_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                assert_eq!(event.event.url.as_deref(), Some("url_str"));
                assert_eq!(event.event.duration.as_deref(), Some("duration_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_RECORDING_READY_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_recording_started_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_recording_started_v1(move |event: MeetingRecordingStartedV1| {
                assert_eq!(event.header.event_type, "vc.meeting.recording_started_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_RECORDING_STARTED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_share_ended_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_share_ended_v1(move |event: MeetingShareEndedV1| {
                assert_eq!(event.header.event_type, "vc.meeting.share_ended_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_SHARE_ENDED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_meeting_share_started_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_meeting_share_started_v1(move |event: MeetingShareStartedV1| {
                assert_eq!(event.header.event_type, "vc.meeting.share_started_v1");
                let meeting = event.event.meeting.as_ref().expect("meeting");
                assert_eq!(meeting.id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_MEETING_SHARE_STARTED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_note_generated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_note_generated_v1(move |event: NoteGeneratedV1| {
                assert_eq!(event.header.event_type, "vc.note.generated_v1");
                assert_eq!(event.event.note_id.as_deref(), Some("note_id_str"));
                assert_eq!(event.event.subscriber_ids.len(), 1);
                assert_eq!(
                    event.event.subscriber_ids[0].user_id.as_deref(),
                    Some("user_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_NOTE_GENERATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_reserve_config_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_reserve_config_updated_v1(move |event: ReserveConfigUpdatedV1| {
                assert_eq!(event.header.event_type, "vc.reserve_config.updated_v1");
                assert_eq!(event.event.scope_id.as_deref(), Some("scope_id_str"));
                assert_eq!(event.event.scope_type, Some(42));
                let approve_config = event.event.approve_config.as_ref().expect("approve_config");
                assert_eq!(approve_config.approval_switch, Some(42));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_RESERVE_CONFIG_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_room_created_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_room_created_v1(move |event: RoomCreatedV1| {
                assert_eq!(event.header.event_type, "vc.room.created_v1");
                let room = event.event.room.as_ref().expect("room");
                assert_eq!(room.room_id.as_deref(), Some("room_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_ROOM_CREATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_room_deleted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_room_deleted_v1(move |event: RoomDeletedV1| {
                assert_eq!(event.header.event_type, "vc.room.deleted_v1");
                let room = event.event.room.as_ref().expect("room");
                assert_eq!(room.room_id.as_deref(), Some("room_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_ROOM_DELETED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_room_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_room_updated_v1(move |event: RoomUpdatedV1| {
                assert_eq!(event.header.event_type, "vc.room.updated_v1");
                let room = event.event.room.as_ref().expect("room");
                assert_eq!(room.room_id.as_deref(), Some("room_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_ROOM_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_room_level_created_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_room_level_created_v1(move |event: RoomLevelCreatedV1| {
                assert_eq!(event.header.event_type, "vc.room_level.created_v1");
                let room_level = event.event.room_level.as_ref().expect("room_level");
                assert_eq!(
                    room_level.room_level_id.as_deref(),
                    Some("room_level_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_ROOM_LEVEL_CREATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_room_level_deleted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_room_level_deleted_v1(move |event: RoomLevelDeletedV1| {
                assert_eq!(event.header.event_type, "vc.room_level.deleted_v1");
                assert_eq!(
                    event.event.room_level_id.as_deref(),
                    Some("room_level_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_ROOM_LEVEL_DELETED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_vc_room_level_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_vc_room_level_updated_v1(move |event: RoomLevelUpdatedV1| {
                assert_eq!(event.header.event_type, "vc.room_level.updated_v1");
                let room_level = event.event.room_level.as_ref().expect("room_level");
                assert_eq!(
                    room_level.room_level_id.as_deref(),
                    Some("room_level_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(VC_ROOM_LEVEL_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    /// 事件串常量与 Go v3.12.2 dispatcher 赋值行逐字一致。
    #[test]
    fn typed_vc_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::VC_BOT_MEETING_ACTIVITY_V1,
            "vc.bot.meeting_activity_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_BOT_MEETING_ENDED_V1,
            "vc.bot.meeting_ended_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_BOT_MEETING_INVITED_V1,
            "vc.bot.meeting_invited_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_BOT_MEETING_STARTED_V1,
            "vc.bot.meeting_started_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_ALL_MEETING_ENDED_V1,
            "vc.meeting.all_meeting_ended_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_ALL_MEETING_STARTED_V1,
            "vc.meeting.all_meeting_started_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_JOIN_MEETING_V1,
            "vc.meeting.join_meeting_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_LEAVE_MEETING_V1,
            "vc.meeting.leave_meeting_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_MEETING_ENDED_V1,
            "vc.meeting.meeting_ended_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_MEETING_STARTED_V1,
            "vc.meeting.meeting_started_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_PARTICIPANT_MEETING_ENDED_V1,
            "vc.meeting.participant_meeting_ended_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_RECORDING_ENDED_V1,
            "vc.meeting.recording_ended_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_RECORDING_READY_V1,
            "vc.meeting.recording_ready_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_RECORDING_STARTED_V1,
            "vc.meeting.recording_started_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_SHARE_ENDED_V1,
            "vc.meeting.share_ended_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_MEETING_SHARE_STARTED_V1,
            "vc.meeting.share_started_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_NOTE_GENERATED_V1,
            "vc.note.generated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_RESERVE_CONFIG_UPDATED_V1,
            "vc.reserve_config.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_ROOM_CREATED_V1,
            "vc.room.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_ROOM_DELETED_V1,
            "vc.room.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_ROOM_UPDATED_V1,
            "vc.room.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_ROOM_LEVEL_CREATED_V1,
            "vc.room_level.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_ROOM_LEVEL_DELETED_V1,
            "vc.room_level.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::VC_ROOM_LEVEL_UPDATED_V1,
            "vc.room_level.updated_v1"
        );
    }

    /// typed 与 `register_raw` 同 key 重复注册返回 Err（双向）。
    #[test]
    fn typed_vc_duplicate_register_fails() {
        struct Noop;
        impl crate::ws_client::EventHandler for Noop {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                Ok(())
            }
        }
        let err = EventDispatcherHandler::builder()
            .register_vc_bot_meeting_activity_v1(|_event: BotMeetingActivityV1| Ok(()))
            .expect("first")
            .register_raw("vc.bot.meeting_activity_v1", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("vc.bot.meeting_activity_v1", Noop)
            .expect("raw first")
            .register_vc_bot_meeting_activity_v1(|_event: BotMeetingActivityV1| Ok(()))
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
