use crate::config::Config;

/// 用户组管理服务
#[derive(Debug)]
pub struct GroupService {
    config: Config,
}

impl GroupService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::trait_system::Service for GroupService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "group"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
