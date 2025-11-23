//! 复制应用模块

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

/// 复制多维表格请求
#[derive(Clone)]
pub struct CopyAppRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 待复制的多维表格的 app_token
    pub app_token: String,
    /// 复制的多维表格 App 名字
    pub name: Option<String>,
    /// 复制的多维表格所在文件夹的 token
    pub folder_token: Option<String>,
    /// 时区
    pub time_zone: Option<String>,
}

impl CopyAppRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                Method::POST,
                COPY_APP.to_string(),
            ),
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
#[derive(Clone)]
pub struct CopyAppResponse {
    /// 复制的多维表格的 app 信息
    pub app: CopyAppResponseData,
}

/// 复制应用响应数据
#[derive(Clone)]
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