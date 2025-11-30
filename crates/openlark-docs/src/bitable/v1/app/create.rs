//! 创建多维表格模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建多维表格请求
#[derive(Clone)]
pub struct CreateAppRequest {
    api_request: ApiRequest<CreateAppResponse>,
    name: String,
    folder_token: Option<String>,
    time_zone: Option<String>,
}

impl CreateAppRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Post)
                .api_path("/open-apis/bitable/v1/apps".to_string())
                .config(config)
                .build(),
            name: String::new(),
            folder_token: None,
            time_zone: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    pub async fn execute(mut self) -> SDKResult<CreateAppResponse> {
        let body = serde_json::json!({
            "name": self.name,
            "folder_token": self.folder_token,
            "time_zone": self.time_zone
        });

        self.api_request = self.api_request.body(serde_json::to_vec(&body)?);

        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

/// 创建应用响应
#[derive(Clone)]
pub struct CreateAppResponse {
    pub app: CreateAppResponseData,
}

/// 创建应用响应数据
#[derive(Clone)]
pub struct CreateAppResponseData {
    pub app_token: String,
    pub name: String,
    pub revision: i32,
    pub url: String,
}

impl ApiResponseTrait for CreateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
