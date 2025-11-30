//! 复制多维表格模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 复制多维表格请求
#[derive(Clone)]
pub struct CopyAppRequest {
    api_request: ApiRequest<CopyAppResponse>,
    app_token: String,
    name: String,
}

impl CopyAppRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Post)
                .api_path("/open-apis/bitable/v1/apps/{}/copy".to_string())
                .config(config)
                .build(),
            app_token: String::new(),
            name: String::new(),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub async fn execute(mut self) -> SDKResult<CopyAppResponse> {
        let path = format!("/open-apis/bitable/v1/apps/{}/copy", self.app_token);
        self.api_request = self.api_request.api_path(path);

        let body = serde_json::json!({
            "name": self.name
        });

        self.api_request = self.api_request.body(serde_json::to_vec(&body)?);

        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

/// 复制多维表格响应
#[derive(Clone)]
pub struct CopyAppResponse {
    pub app: CopyAppResponseData,
}

/// 复制多维表格响应数据
#[derive(Clone)]
pub struct CopyAppResponseData {
    pub app_token: String,
    pub name: String,
    pub revision: i32,
    pub url: String,
}

impl ApiResponseTrait for CopyAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}