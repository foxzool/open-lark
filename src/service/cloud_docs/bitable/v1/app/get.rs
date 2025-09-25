use reqwest::Method;
use serde::Deserialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::AppService;

impl AppService {
    /// 获取多维表格元数据
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/get>
    pub async fn get(
        &self,
        request: GetAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAppResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = BITABLE_V1_APP_GET.replace("{app_token}", &request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 获取多维表格元数据请求
#[derive(Debug, Default)]
pub struct GetAppRequest {
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    app_token: String,
}

impl GetAppRequest {
    pub fn builder() -> GetAppRequestBuilder {
        GetAppRequestBuilder::default()
    }

    /// 创建获取多维表格元数据请求
    pub fn new(app_token: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
        }
    }
}

#[derive(Default)]
pub struct GetAppRequestBuilder {
    request: GetAppRequest,
}

impl GetAppRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    pub fn build(self) -> GetAppRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    GetAppRequestBuilder,
    AppService,
    GetAppRequest,
    BaseResponse<GetAppResponse>,
    get
);

#[derive(Deserialize, Debug)]
pub struct GetAppResponse {
    /// 多维表格的 app 信息
    pub app: GetAppResponseData,
}

#[derive(Deserialize, Debug)]
pub struct GetAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号（对多维表格进行修改时更新，如新增、删除数据表，修改数据表名等，初始为1，
    /// 每次更新+1）
    pub revision: i32,
    /// 多维表格是否开启了高级权限。取值包括：
    ///
    /// - true：表示开启了高级权限
    /// - false：表示关闭了高级权限
    pub is_advanced: bool,
    /// 文档时区
    pub time_zone: String,
}

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_get_app_request() {
        let request = GetAppRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
    }

    #[test]
    fn test_get_app_request_new() {
        let request = GetAppRequest::new("bascnmBA*****yGehy8");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
    }

    #[test]
    fn test_get_app_request_builder_default() {
        let builder = GetAppRequestBuilder::default();
        let request = builder.build();
        assert_eq!(request.app_token, "");
    }

    #[test]
    fn test_get_app_request_builder_multiple_calls() {
        let request = GetAppRequest::builder()
            .app_token("first_token")
            .app_token("final_token")
            .build();
        assert_eq!(request.app_token, "final_token");
    }

    #[test]
    fn test_get_app_request_default() {
        let request = GetAppRequest::default();
        assert_eq!(request.app_token, "");
    }

    #[test]
    fn test_get_app_request_debug() {
        let request = GetAppRequest::new("test_app_token");
        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("GetAppRequest"));
        assert!(debug_str.contains("test_app_token"));
    }

    #[test]
    fn test_get_app_request_with_string_types() {
        let owned_string = String::from("owned_token");
        let request1 = GetAppRequest::new(owned_string);
        assert_eq!(request1.app_token, "owned_token");

        let string_ref = "ref_token";
        let request2 = GetAppRequest::new(string_ref);
        assert_eq!(request2.app_token, "ref_token");

        let request3 = GetAppRequest::builder()
            .app_token(String::from("builder_token"))
            .build();
        assert_eq!(request3.app_token, "builder_token");
    }

    #[test]
    fn test_get_app_request_with_special_characters() {
        let special_token = "app_token_with_特殊字符_123";
        let request = GetAppRequest::new(special_token);
        assert_eq!(request.app_token, special_token);
    }

    #[test]
    fn test_get_app_request_with_empty_token() {
        let request = GetAppRequest::new("");
        assert_eq!(request.app_token, "");
    }

    #[test]
    fn test_get_app_response_deserialization() {
        let json = r#"{
            "app": {
                "app_token": "bascnmBA*****yGehy8",
                "name": "测试多维表格",
                "revision": 5,
                "is_advanced": true,
                "time_zone": "Asia/Shanghai"
            }
        }"#;

        let response: GetAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.app.app_token, "bascnmBA*****yGehy8");
        assert_eq!(response.app.name, "测试多维表格");
        assert_eq!(response.app.revision, 5);
        assert!(response.app.is_advanced);
        assert_eq!(response.app.time_zone, "Asia/Shanghai");
    }

    #[test]
    fn test_get_app_response_data_debug() {
        let app_data = GetAppResponseData {
            app_token: "test_token".to_string(),
            name: "Test App".to_string(),
            revision: 1,
            is_advanced: false,
            time_zone: "UTC".to_string(),
        };

        let debug_str = format!("{:?}", app_data);
        assert!(debug_str.contains("GetAppResponseData"));
        assert!(debug_str.contains("test_token"));
        assert!(debug_str.contains("Test App"));
        assert!(debug_str.contains("UTC"));
    }

    #[test]
    fn test_get_app_response_debug() {
        let response = GetAppResponse {
            app: GetAppResponseData {
                app_token: "debug_token".to_string(),
                name: "Debug App".to_string(),
                revision: 3,
                is_advanced: true,
                time_zone: "America/New_York".to_string(),
            },
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("GetAppResponse"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("Debug App"));
    }

    #[test]
    fn test_get_app_response_with_false_advanced() {
        let json = r#"{
            "app": {
                "app_token": "basic_app_token",
                "name": "Basic App",
                "revision": 1,
                "is_advanced": false,
                "time_zone": "UTC"
            }
        }"#;

        let response: GetAppResponse = serde_json::from_str(json).unwrap();
        assert!(!response.app.is_advanced);
        assert_eq!(response.app.revision, 1);
    }

    #[test]
    fn test_get_app_response_with_different_timezones() {
        let timezones = vec![
            ("UTC", "UTC"),
            ("Asia/Shanghai", "Asia/Shanghai"),
            ("America/New_York", "America/New_York"),
            ("Europe/London", "Europe/London"),
        ];

        for (tz_input, tz_expected) in timezones {
            let json = format!(
                r#"{{
                    "app": {{
                        "app_token": "test_token",
                        "name": "Test",
                        "revision": 1,
                        "is_advanced": false,
                        "time_zone": "{}"
                    }}
                }}"#,
                tz_input
            );

            let response: GetAppResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.app.time_zone, tz_expected);
        }
    }

    #[test]
    fn test_get_app_response_data_format() {
        let format = GetAppResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
    }

    #[test]
    fn test_get_app_response_with_large_revision() {
        let json = r#"{
            "app": {
                "app_token": "test",
                "name": "Test",
                "revision": 999999,
                "is_advanced": true,
                "time_zone": "UTC"
            }
        }"#;

        let response: GetAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.app.revision, 999999);
    }

    #[test]
    fn test_get_app_response_with_unicode_name() {
        let json = r#"{
            "app": {
                "app_token": "unicode_test",
                "name": "测试应用📱💼",
                "revision": 2,
                "is_advanced": false,
                "time_zone": "Asia/Tokyo"
            }
        }"#;

        let response: GetAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.app.name, "测试应用📱💼");
        assert_eq!(response.app.time_zone, "Asia/Tokyo");
    }

    #[test]
    fn test_get_app_response_with_zero_revision() {
        let json = r#"{
            "app": {
                "app_token": "zero_rev",
                "name": "Zero Revision App",
                "revision": 0,
                "is_advanced": false,
                "time_zone": "UTC"
            }
        }"#;

        let response: GetAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.app.revision, 0);
    }

    #[test]
    fn test_memory_efficiency() {
        let request = GetAppRequest::new("test");
        let size = std::mem::size_of_val(&request);
        assert!(size > 0);
        assert!(size < 1024);
    }
}
