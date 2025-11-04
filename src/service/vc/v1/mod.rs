use crate::core::config::Config;
pub mod meeting;
pub mod recording;
pub mod reserve;
pub mod room;
// VC v1 事件模块
pub mod p2_vc_meeting_ended_v1;
pub mod p2_vc_meeting_participant_joined_v1;
pub mod p2_vc_meeting_participant_left_v1;
pub mod p2_vc_meeting_started_v1;
use meeting::MeetingService;
use recording::RecordingService;
use reserve::ReserveService;
use room::RoomService;
/// VC v1服务
pub struct V1 {
}

impl V1 {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}