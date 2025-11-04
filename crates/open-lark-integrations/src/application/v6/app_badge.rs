use open_lark_core::core::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    trait_system::Service,
};

/// 应用红点服务
pub struct AppBadgeService {
    config: Config,


impl AppBadgeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // TODO: 实现应用红点相关API


impl Service for AppBadgeService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "AppBadgeService"
    }

    fn service_version() -> &'static str {
        "v1"
    }
