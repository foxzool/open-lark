use open_lark_core::core::config::Config;

pub mod get;
pub mod query_availability;
pub mod reply;

/// 会议室日程服务
pub struct MeetingRoomEventService {
    pub config: Config,
}

impl MeetingRoomEventService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
