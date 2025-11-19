use crate::{
    api::ApiRequest, api::ApiResponseTrait, config::Config,
    constants::AccessTokenType, http::Transport,
};
use serde::{Deserialize, Serialize};

/// 职务服务
#[derive(Debug)]
pub struct JobTitleService {
    config: Config,
}

impl JobTitleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::trait_system::Service for JobTitleService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "job_title"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
