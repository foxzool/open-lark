use crate::{
    api::ApiRequest, api::ApiResponseTrait, config::Config,
    constants::AccessTokenType, http::Transport,
};
use serde::{Deserialize, Serialize};

/// Ufunctional_role_member 服务
#[derive(Debug)]
pub struct Ufunctional_role_memberService {
    config: Config,
}

impl Ufunctional_role_memberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Service for Ufunctional_role_memberService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "functional_role_member"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
