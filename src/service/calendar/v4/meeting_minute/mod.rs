use crate::core::config::Config;

pub mod create;

/// 会议纪要服务
pub struct MeetingMinuteService {
    pub config: Config,
}

impl MeetingMinuteService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
