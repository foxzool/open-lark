use openlark_core::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    trait_system::Service,
};

/// 应用使用情况服务
pub struct AppUsageService {
    config: Config,


impl AppUsageService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // TODO: 实现应用使用情况相关API


impl Service for AppUsageService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "AppUsageService"
    }

    fn service_version() -> &'static str {
        "v1"
    }
