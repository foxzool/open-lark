use reqwest::blocking::Response;
use crate::core::config::Config;

use crate::core::constants::AppType;
use crate::core::error::LarkAPIError;
use crate::core::http::Transport;
use crate::core::model::*;
use crate::core::token::verify;

pub struct LarkClient {
   pub config: Config,
}

impl LarkClient {
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        let config = Config {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            ..Default::default()
        };

        Self { config }
    }

    pub fn with_app_type(mut self, app_type: AppType) -> Self {
        self.config.app_type = app_type;
        self
    }

    pub fn with_marketplace_app(mut self) -> Self {
        self.config.app_type = AppType::Marketplace;
        self
    }

    pub fn builder() -> LarkClientBuilder {
        LarkClientBuilder::default()
    }

    pub fn request<T: BaseResponseTrait>(
        &self,
        mut request: BaseRequest,
        option: Option<RequestOption>,
    ) -> Result<BaseResponse<T>, LarkAPIError> {
        let mut option = option.unwrap_or_default();

        verify(&self.config, &mut request, &mut option)?;

        // 发起请求
        let raw = Transport::execute(&self.config, &request, Some(option))?;

        BaseResponse::from_response(raw)
    }

    pub fn request_raw(
        &self,
        mut request: BaseRequest,
        option: Option<RequestOption>,
    ) -> Result<Response, LarkAPIError> {
        let mut option = option.unwrap_or_default();

        verify(&self.config, &mut request, &mut option)?;

        // 发起请求
        Transport::execute(&self.config, &request, Some(option))
    }
}

#[derive(Default)]
pub struct LarkClientBuilder {
    config: Config,
}

impl LarkClientBuilder {
    pub fn app_id(mut self, app_id: String) -> Self {
        self.config.app_id = app_id;
        self
    }

    pub fn app_secret(mut self, app_secret: String) -> Self {
        self.config.app_secret = app_secret;
        self
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.config.base_url = domain;
        self
    }

    pub fn timeout(mut self, timeout: f32) -> Self {
        self.config.timeout = Some(timeout);
        self
    }

    pub fn app_type(mut self, app_type: AppType) -> Self {
        self.config.app_type = app_type;
        self
    }

    pub fn app_ticket(mut self, app_ticket: String) -> Self {
        self.config.app_ticket = Some(app_ticket);
        self
    }

    pub fn enable_set_token(mut self, enable_set_token: bool) -> Self {
        self.config.enable_set_token = enable_set_token;
        self
    }

    pub fn build(self) -> LarkClient {
        LarkClient {
            config: self.config,
        }
    }
}
