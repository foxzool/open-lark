use openlark_core::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    trait_system::Service,
};

/// 应用权限服务
pub struct ScopeService {
    config: Config,


impl ScopeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // TODO: 实现应用权限相关API


impl Service for ScopeService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ScopeService"
    }

    fn service_version() -> &'static str {
        "v1"
    }
