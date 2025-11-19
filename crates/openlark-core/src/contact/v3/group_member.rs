use crate::{
    api::ApiRequest, api::ApiResponseTrait, config::Config,
    constants::AccessTokenType, http::Transport,
};
use serde::{Deserialize, Serialize};

/// group_member 服务
#[derive(Debug)]
pub struct UgroupUmemberService {
    config: Config,
}

impl UgroupUmemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::trait_system::Service for UgroupUmemberService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "group_member"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
