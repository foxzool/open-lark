use crate::core::config::Config;

pub mod create;
pub mod delete;

/// 请假日程服务
pub struct TimeoffEventService {
    pub config: Config,
}

impl TimeoffEventService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
