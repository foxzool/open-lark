use crate::config::Config;

/// 工作城市服务
#[derive(Debug)]
pub struct WorkCityService {
    config: Config,
}

impl WorkCityService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::trait_system::Service for WorkCityService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "work_city"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
