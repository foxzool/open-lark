use crate::{
    api::ApiRequest, api::ApiResponseTrait, config::Config,
    constants::AccessTokenType, http::Transport,
};
use serde::{Deserialize, Serialize};

/// Uunit 服务
#[derive(Debug)]
pub struct UunitService {
    config: Config,
}

impl UunitService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::trait_system::Service for UunitService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "unit"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
