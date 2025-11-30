#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::App,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 复制多维表格请求
#[derive(Clone)]
pub struct CopyAppRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 复制的多维表格 App 名字
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 复制的多维表格所在文件夹的 token
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_token: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
}

impl CopyAppRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps/{}/copy".to_string()),
            app_token: String::new(),
            name: None,
            folder_token: None,
            time_zone: None,
        }
    }

    pub fn builder() -> CopyAppRequestBuilder {
        CopyAppRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CopyAppRequestBuilder {
    request: CopyAppRequest,
}

impl CopyAppRequestBuilder {
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

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.request.time_zone = Some(time_zone.into());
        self
    }

    pub fn build(self) -> CopyAppRequest {
        self.request
    }
}

#[derive(Serialize)]
struct CopyAppRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
}

/// 复制应用响应
#[derive(Clone, Serialize, Deserialize)]
pub struct CopyAppResponse {
    /// 复制的多维表格的 app 信息
    pub app: CopyAppResponseData,
}

/// 复制应用响应数据
#[derive(Clone, Serialize, Deserialize)]
pub struct CopyAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格的链接
    pub url: String,
}

/// 复制多维表格
pub async fn copy_app(
    request: CopyAppRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CopyAppResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_APP_COPY
        .replace("{app_token}", &request.app_token);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置请求体
    let body = CopyAppRequestBody {
        name: request.name,
        folder_token: request.folder_token,
        time_zone: request.time_zone,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<CopyAppResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

impl ApiResponseTrait for CopyAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_copy_app_request() {
        let request = CopyAppRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .name("复制的多维表格")
            .folder_token("fldcnmBA*****yGehy8")
            .time_zone("Asia/Shanghai")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.name, Some("复制的多维表格".to_string()));
        assert_eq!(request.folder_token, Some("fldcnmBA*****yGehy8".to_string()));
        assert_eq!(request.time_zone, Some("Asia/Shanghai".to_string()));
    }

    #[test]
    fn test_copy_app_request_body_serialization() {
        let body = CopyAppRequestBody {
            name: Some("复制的多维表格".to_string()),
            folder_token: Some("fldcnmBA*****yGehy8".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "复制的多维表格",
            "folder_token": "fldcnmBA*****yGehy8",
            "time_zone": "Asia/Shanghai"
        });

        assert_eq!(serialized, expected);
    }
}