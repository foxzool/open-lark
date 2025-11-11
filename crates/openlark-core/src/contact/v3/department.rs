use crate::config::Config;

/// department 服务
#[derive(Debug)]
pub struct UdepartmentService {
    config: Config,
}

impl UdepartmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::trait_system::Service for UdepartmentService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "department"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
