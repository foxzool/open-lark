//! 创建应用模块

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
use super::AppService;

/// 创建多维表格请求
#[derive(Clone)]
pub struct CreateAppRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的文件夹token
    pub folder_token: Option<String>,
    /// 时区
    pub time_zone: Option<String>,
    /// 多维表格的模板
    pub app_template_id: Option<String>,
    /// 多维表格的模板类型
    pub app_template_type: Option<String>,
}

impl CreateAppRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::POST,
                CREATE_APP.to_string(),
            ),
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
#[derive(Clone)]
pub struct CreateAppResponse {
    /// 创建的多维表格的 app 信息
    pub app: CreateAppResponseData,
}

/// 创建应用响应数据
#[derive(Clone)]
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