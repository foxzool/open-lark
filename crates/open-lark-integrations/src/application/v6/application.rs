use open_lark_core::core::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    trait_system::Service,
};

/// 应用服务
pub struct ApplicationService {
    config: Config,


impl ApplicationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // TODO: 实现应用相关API


impl Service for ApplicationService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ApplicationService"
    }

    fn service_version() -> &'static str {
        "v1"
    }
