use crate::core::constants::{APPLICATION_JSON, CONTENT_TYPE};
use crate::core::enum_type::AppType;
use crate::core::error::LarkAPIError;
use crate::core::http::Transport;
use crate::core::model::*;
use crate::core::token::verify;

pub struct LarkClient {
    config: Config,
}

impl LarkClient {
    pub fn builder() -> LarkClientBuilder {
        LarkClientBuilder::default()
    }

    pub fn request(&self, mut request: BaseRequest, option: Option<RequestOption>) -> Result<BaseResponse, LarkAPIError> {
        let mut option = option.unwrap_or_default();

        verify(&self.config, &mut request, &mut option)?;

        // 发起请求
        let raw_resp = Transport::execute(&self.config, &request, Some(option));

        let mut resp = BaseResponse::default();
        let content_type = raw_resp.headers.get(CONTENT_TYPE);
        if let Some(content_type) = content_type {
            if content_type.starts_with(APPLICATION_JSON) {
             resp = serde_json::from_slice::<BaseResponse>(raw_resp.content.clone().unwrap().as_ref()).unwrap();
            }
        } else if   200 <= raw_resp.status_code && raw_resp.status_code  < 300 {
            resp.code = Some(0);
        };
        resp.raw = Some(raw_resp.clone());

        Ok(resp)
    }
}

#[derive(Default)]
pub struct LarkClientBuilder {
    config: Config,
}

impl LarkClientBuilder {
    pub fn app_id(mut self, app_id: String) -> Self {
        self.config.app_id = Some(app_id);
        self
    }

    pub fn app_secret(mut self, app_secret: String) -> Self {
        self.config.app_secret = Some(app_secret);
        self
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.config.domain = domain;
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
            config: self.config
        }
    }
}
