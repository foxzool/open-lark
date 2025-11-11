use openlark_core::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    trait_system::Service,
};

/// 应用商店付费信息服务
pub struct AppstorePaidInfoService {
    config: Config,


impl AppstorePaidInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // TODO: 实现应用商店付费信息相关API


impl Service for AppstorePaidInfoService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "AppstorePaidInfoService"
    }

    fn service_version() -> &'static str {
        "v1"
    }
