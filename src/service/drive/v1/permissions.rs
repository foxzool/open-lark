use crate::core::config::Config;

pub struct PermissionsService {
    config: Config,
}


impl PermissionsService {
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }
}