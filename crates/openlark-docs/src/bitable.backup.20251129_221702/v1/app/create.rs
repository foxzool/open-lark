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

/// 创建多维表格请求
#[derive(Clone)]
pub struct CreateAppRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的名字
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    /// 多维表格的文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_token: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    /// 多维表格的模板
    #[serde(skip_serializing_if = "Option::is_none")]
    app_template_id: Option<String>,
    /// 多维表格的模板类型
    #[serde(skip_serializing_if = "Option::is_none")]
    app_template_type: Option<String>,
}

impl CreateAppRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps".to_string()),
            name: String::new(),
            folder_token: None,
            time_zone: None,
            app_template_id: None,
            app_template_type: None,
        }
    }

    pub fn builder() -> CreateAppRequestBuilder {
        CreateAppRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateAppRequestBuilder {
    request: CreateAppRequest,
}

impl CreateAppRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
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

    pub fn app_template_id(mut self, app_template_id: impl Into<String>) -> Self {
        self.request.app_template_id = Some(app_template_id.into());
        self
    }

    pub fn app_template_type(mut self, app_template_type: impl Into<String>) -> Self {
        self.request.app_template_type = Some(app_template_type.into());
        self
    }

    pub fn build(self) -> CreateAppRequest {
        self.request
    }
}

#[derive(Serialize)]
struct CreateAppRequestBody {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_template_type: Option<String>,
}

/// 创建应用响应
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateAppResponse {
    /// 创建的多维表格的 app 信息
    pub app: CreateAppResponseData,
}

/// 创建应用响应数据
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格的链接
    pub url: String,
}

/// 创建多维表格
pub async fn create_app(
    request: CreateAppRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateAppResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_APP_CREATE;
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置请求体
    let body = CreateAppRequestBody {
        name: request.name,
        folder_token: request.folder_token,
        time_zone: request.time_zone,
        app_template_id: request.app_template_id,
        app_template_type: request.app_template_type,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<CreateAppResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

impl ApiResponseTrait for CreateAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_app_request() {
        let request = CreateAppRequest::builder()
            .name("测试多维表格")
            .folder_token("fldcnmBA*****yGehy8")
            .time_zone("Asia/Shanghai")
            .build();

        assert_eq!(request.name, "测试多维表格");
        assert_eq!(request.folder_token, Some("fldcnmBA*****yGehy8".to_string()));
        assert_eq!(request.time_zone, Some("Asia/Shanghai".to_string()));
    }

    #[test]
    fn test_create_app_request_body_serialization() {
        let body = CreateAppRequestBody {
            name: "测试多维表格".to_string(),
            folder_token: Some("fldcnmBA*****yGehy8".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
            app_template_id: None,
            app_template_type: None,
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "测试多维表格",
            "folder_token": "fldcnmBA*****yGehy8",
            "time_zone": "Asia/Shanghai"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_timezone_serialization() {
        let tz = "Asia/Shanghai";
        let body = CreateAppRequestBody {
            name: "时区测试".to_string(),
            folder_token: None,
            time_zone: Some(tz.to_string()),
            app_template_id: None,
            app_template_type: None,
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "时区测试",
            "time_zone": tz
        });

        assert_eq!(serialized, expected);
    }
}