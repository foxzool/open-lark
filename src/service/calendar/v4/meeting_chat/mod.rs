use crate::core::config::Config;

pub mod create;
pub mod delete;

/// 会议群服务
pub struct MeetingChatService {
    pub config: Config,
}

impl MeetingChatService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
