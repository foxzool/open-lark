//! 重置用户密码 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct ResetPasswordBuilder {
    user_id: String,
    new_password: String,
    config: Config,
}

impl ResetPasswordBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            user_id: String::new(),
            new_password: String::new(),
            config,
        }
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    pub fn new_password(mut self, new_password: impl Into<String>) -> Self {
        self.new_password = new_password.into();
        self
    }

    pub async fn execute(self) -> SDKResult<ResetPasswordResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ResetPasswordResponse> {
        validate_required!(self.user_id, "用户ID不能为空");
        validate_required!(self.new_password, "新密码不能为空");

        let request_body = ResetPasswordRequest {
            user_id: self.user_id,
            new_password: self.new_password,
        };

        let api_request: ApiRequest<ResetPasswordResponse> =
            ApiRequest::post("/open-apis/admin/v1/password/reset")
                .body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("重置用户密码", "响应数据为空")
        })
    }
}

#[derive(Debug, Serialize)]
struct ResetPasswordRequest {
    user_id: String,
    new_password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResetPasswordResponse {
    pub result: String,
}

impl ApiResponseTrait for ResetPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
