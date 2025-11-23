//! 获取应用信息模块

use openlark_core::{
    core::{
        BaseResponse,
        ResponseFormat,
        api::ApiResponseTrait,
    },
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取应用请求
#[derive(Clone)]
pub struct GetAppRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
}

impl GetAppRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::GET,
                GET_APP.to_string(),
            ),
            app_token: String::new(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_app_request() {
        let request = GetAppRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
    }
}