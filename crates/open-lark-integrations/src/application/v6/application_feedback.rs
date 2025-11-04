use open_lark_core::core::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    trait_system::Service,
};

/// 应用反馈服务
pub struct ApplicationFeedbackService {
    config: Config,


impl ApplicationFeedbackService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // TODO: 实现应用反馈相关API


impl Service for ApplicationFeedbackService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ApplicationFeedbackService"
    }

    fn service_version() -> &'static str {
        "v1"
    }
