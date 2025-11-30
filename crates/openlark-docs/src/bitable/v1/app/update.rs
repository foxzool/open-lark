//! 更新应用模块

use openlark_core::{
    core::{
        BaseResponse,
        ResponseFormat,
    },
    
    
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新应用请求
#[derive(Clone)]
pub struct UpdateAppRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: Option<String>,
    /// 时区
    pub time_zone: Option<String>,
}

impl UpdateAppRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                HttpMethod::PUT,
                UPDATE_APP.to_string(),
            ),
            app_token: String::new(),
            name: None,
            time_zone: None,
        }
    }

    pub fn builder() -> UpdateAppRequestBuilder {
        UpdateAppRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateAppRequestBuilder {
    request: UpdateAppRequest,
}

impl UpdateAppRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.request.time_zone = Some(time_zone.into());
        self
    }

    pub fn build(self) -> UpdateAppRequest {
        self.request
    }
}

#[derive(Serialize)]
struct UpdateAppRequestBody {
    #[serde(skip_serializing_if = Option::is_none)]
    name: Option<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    time_zone: Option<String>,
}

/// 更新应用响应
#[derive(Clone)]
pub struct UpdateAppResponse {
    /// 更新的应用信息
    pub app: UpdateAppResponseData,
}

/// 更新应用响应数据
#[derive(Clone)]
pub struct UpdateAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格的链接
    pub url: String,
}

impl ApiResponseTrait for UpdateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

