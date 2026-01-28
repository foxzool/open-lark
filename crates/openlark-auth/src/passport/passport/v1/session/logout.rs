//! 退出登录
//!
//! docPath: https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/logout

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 退出登录请求
pub struct LogoutRequest {
    config: Config,
    user_id: Option<String>,
}

impl LogoutRequest {
    pub fn new(config: Config) -> Self {
        Self { config, user_id: None }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub async fn execute(self) -> SDKResult<LogoutResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<LogoutResponse> {
        let req: ApiRequest<LogoutResponse> =
            ApiRequest::post("/open-apis/passport/v1/sessions/logout");

        let response = Transport::request(req, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("logout", "响应数据为空")
        })
    }
}

/// 退出登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub result: String,
}

impl ApiResponseTrait for LogoutResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
