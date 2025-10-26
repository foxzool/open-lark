use crate::core::config::Config;

/// 用户管理服务
#[derive(Debug)]
pub struct UserService {
    config: Config,
}

impl UserService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::core::trait_system::Service for UserService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "user"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
