use crate::{
    api::ApiRequest, api::ApiResponseTrait, config::Config,
    constants::AccessTokenType, http::Transport,
};
use serde::{Deserialize, Serialize};

/// job_family 服务
#[derive(Debug)]
pub struct UjobUfamilyService {
    config: Config,
}

impl UjobUfamilyService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::trait_system::Service for UjobUfamilyService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "job_family"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
