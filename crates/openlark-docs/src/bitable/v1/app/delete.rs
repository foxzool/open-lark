//! 删除多维表格模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除应用请求
#[derive(Clone)]
pub struct DeleteAppRequest {
    api_request: ApiRequest<DeleteAppResponse>,
    app_token: String,
}

impl DeleteAppRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Delete)
                .api_path("/open-apis/bitable/v1/apps".to_string())
                .config(config)
                .build(),
            app_token: String::new(),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub async fn execute(mut self) -> SDKResult<DeleteAppResponse> {
        let path = format!("/open-apis/bitable/v1/apps/{}", self.app_token);
        self.api_request = self.api_request.api_path(path);

        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

/// 删除应用响应
#[derive(Clone)]
pub struct DeleteAppResponse {
    pub success: bool,
}

impl ApiResponseTrait for DeleteAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
