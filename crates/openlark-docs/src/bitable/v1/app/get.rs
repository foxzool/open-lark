//! 获取应用信息模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取应用请求
#[derive(Clone)]
pub struct GetAppRequest {
    api_request: ApiRequest<GetAppResponse>,
    /// 多维表格的 app_token
    pub app_token: String,
}

impl GetAppRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Get)
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

    pub async fn execute(mut self) -> SDKResult<GetAppResponse> {
        // 构建API路径
        let path = format!(/open-apis/bitable/v1/apps/{}, self.app_token);

        // 更新API路径
        self.api_request = self.api_request.api_path(path);

        // 发送请求
        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }

    pub fn builder() -> GetAppRequestBuilder {
        GetAppRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetAppRequestBuilder {
    request: GetAppRequest,
}

impl GetAppRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn build(self) -> GetAppRequest {
        self.request
    }
}

/// 获取应用响应
#[derive(Clone)]
pub struct GetAppResponse {
    /// 应用信息
    pub app: GetAppResponseData,
}

/// 获取应用响应数据
#[derive(Clone)]
pub struct GetAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格的链接
    pub url: String,
}

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

