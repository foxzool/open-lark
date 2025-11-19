use crate::{
    api::ApiRequest, api::ApiResponseTrait, config::Config,
    constants::AccessTokenType, http::Transport,
};
use serde::{Deserialize, Serialize};

/// job_level 服务
#[derive(Debug)]
pub struct UjobUlevelService {
    config: Config,
}

impl UjobUlevelService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::trait_system::Service for UjobUlevelService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "job_level"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
