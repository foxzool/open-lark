use crate::core::{config::Config, trait_system::Service};

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

impl Service for TimeoffEventService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "timeoff_event"
    }

    fn service_version() -> &'static str {
        "v4"
    }
}
