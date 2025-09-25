use reqwest::Method;
use serde::{Deserialize, Serialize};

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
    /// 创建多维表格
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create>
    pub async fn create(
        &self,
        request: CreateAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateAppResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = BITABLE_V1_APPS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&CreateAppRequestBody {
            name: request.name,
            folder_token: request.folder_token,
            time_zone: request.time_zone,
        })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 创建多维表格请求
#[derive(Debug, Default)]
pub struct CreateAppRequest {
    api_request: ApiRequest,
    /// 多维表格 App 名字
    name: String,
    /// 多维表格所在文件夹的 token，若不传则默认添加到用户云空间的根目录下
    folder_token: Option<String>,
    /// 时区
    time_zone: Option<String>,
}

impl CreateAppRequest {
    pub fn builder() -> CreateAppRequestBuilder {
        CreateAppRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateAppRequestBuilder {
    request: CreateAppRequest,
}

impl CreateAppRequestBuilder {
    /// 多维表格 App 名字
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = name.to_string();
        self
    }

    /// 多维表格所在文件夹的 token
    pub fn folder_token(mut self, folder_token: impl ToString) -> Self {
        self.request.folder_token = Some(folder_token.to_string());
        self
    }

    /// 时区
    pub fn time_zone(mut self, time_zone: impl ToString) -> Self {
        self.request.time_zone = Some(time_zone.to_string());
        self
    }

    pub fn build(self) -> CreateAppRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    CreateAppRequestBuilder,
    AppService,
    CreateAppRequest,
    BaseResponse<CreateAppResponse>,
    create
);

#[derive(Serialize)]
struct CreateAppRequestBody {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateAppResponse {
    /// 多维表格的 app 信息
    pub app: CreateAppResponseData,
}

#[derive(Deserialize, Debug)]
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
#[allow(unused_variables, unused_unsafe)]
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
        assert_eq!(
            request.folder_token,
            Some("fldcnmBA*****yGehy8".to_string())
        );
        assert_eq!(request.time_zone, Some("Asia/Shanghai".to_string()));
    }

    #[test]
    fn test_create_app_request_body_serialization() {
        let body = CreateAppRequestBody {
            name: "测试多维表格".to_string(),
            folder_token: Some("fldcnmBA*****yGehy8".to_string()),
            time_zone: Some("Asia/Shanghai".to_string()),
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
    fn test_create_app_request_builder_default() {
        let builder = CreateAppRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.name, "");
        assert_eq!(request.folder_token, None);
        assert_eq!(request.time_zone, None);
    }

    #[test]
    fn test_create_app_request_default() {
        let request = CreateAppRequest::default();

        assert_eq!(request.name, "");
        assert_eq!(request.folder_token, None);
        assert_eq!(request.time_zone, None);
    }

    #[test]
    fn test_create_app_request_minimal() {
        let request = CreateAppRequest::builder().name("简单表格").build();

        assert_eq!(request.name, "简单表格");
        assert_eq!(request.folder_token, None);
        assert_eq!(request.time_zone, None);
    }

    #[test]
    fn test_create_app_request_with_folder_only() {
        let request = CreateAppRequest::builder()
            .name("文件夹表格")
            .folder_token("folder123")
            .build();

        assert_eq!(request.name, "文件夹表格");
        assert_eq!(request.folder_token, Some("folder123".to_string()));
        assert_eq!(request.time_zone, None);
    }

    #[test]
    fn test_create_app_request_with_timezone_only() {
        let request = CreateAppRequest::builder()
            .name("时区表格")
            .time_zone("UTC")
            .build();

        assert_eq!(request.name, "时区表格");
        assert_eq!(request.folder_token, None);
        assert_eq!(request.time_zone, Some("UTC".to_string()));
    }

    #[test]
    fn test_create_app_request_builder_chaining() {
        let request = CreateAppRequest::builder()
            .name("链式调用")
            .folder_token("folder456")
            .time_zone("Europe/London")
            .name("更新名称")
            .build();

        assert_eq!(request.name, "更新名称");
        assert_eq!(request.folder_token, Some("folder456".to_string()));
        assert_eq!(request.time_zone, Some("Europe/London".to_string()));
    }

    #[test]
    fn test_create_app_request_debug() {
        let request = CreateAppRequest::builder().name("调试测试").build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CreateAppRequest"));
        assert!(debug_str.contains("调试测试"));
    }

    #[test]
    fn test_create_app_request_with_unicode_name() {
        let unicode_name = "测试表格🚀📊📈";
        let request = CreateAppRequest::builder().name(unicode_name).build();

        assert_eq!(request.name, unicode_name);
    }

    #[test]
    fn test_create_app_request_with_string_types() {
        let owned_string = String::from("拥有字符串");
        let request1 = CreateAppRequest::builder().name(owned_string).build();
        assert_eq!(request1.name, "拥有字符串");

        let string_ref = "引用字符串";
        let request2 = CreateAppRequest::builder().name(string_ref).build();
        assert_eq!(request2.name, "引用字符串");
    }

    #[test]
    fn test_create_app_request_body_with_none_values() {
        let body = CreateAppRequestBody {
            name: "基础表格".to_string(),
            folder_token: None,
            time_zone: None,
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "基础表格"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_create_app_request_body_with_empty_strings() {
        let body = CreateAppRequestBody {
            name: "".to_string(),
            folder_token: Some("".to_string()),
            time_zone: Some("".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "",
            "folder_token": "",
            "time_zone": ""
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_create_app_response_deserialization() {
        let json = r#"{
            "app": {
                "app_token": "bascnmBA*****yGehy8",
                "name": "新建多维表格",
                "revision": 1,
                "url": "https://example.feishu.cn/base/bascnmBA*****yGehy8"
            }
        }"#;

        let response: CreateAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.app.app_token, "bascnmBA*****yGehy8");
        assert_eq!(response.app.name, "新建多维表格");
        assert_eq!(response.app.revision, 1);
        assert_eq!(
            response.app.url,
            "https://example.feishu.cn/base/bascnmBA*****yGehy8"
        );
    }

    #[test]
    fn test_create_app_response_data_debug() {
        let app_data = CreateAppResponseData {
            app_token: "test_token".to_string(),
            name: "Test App".to_string(),
            revision: 1,
            url: "https://test.url".to_string(),
        };

        let debug_str = format!("{:?}", app_data);
        assert!(debug_str.contains("CreateAppResponseData"));
        assert!(debug_str.contains("test_token"));
        assert!(debug_str.contains("Test App"));
        assert!(debug_str.contains("https://test.url"));
    }

    #[test]
    fn test_create_app_response_debug() {
        let response = CreateAppResponse {
            app: CreateAppResponseData {
                app_token: "debug_token".to_string(),
                name: "Debug App".to_string(),
                revision: 2,
                url: "https://debug.url".to_string(),
            },
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("CreateAppResponse"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("Debug App"));
    }

    #[test]
    fn test_create_app_response_data_format() {
        let format = CreateAppResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
    }

    #[test]
    fn test_create_app_response_with_different_revisions() {
        let revisions = vec![0, 1, 5, 100, 999999];

        for revision in revisions {
            let json = format!(
                r#"{{
                    "app": {{
                        "app_token": "test_token",
                        "name": "Test App",
                        "revision": {},
                        "url": "https://test.url"
                    }}
                }}"#,
                revision
            );

            let response: CreateAppResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.app.revision, revision);
        }
    }

    #[test]
    fn test_create_app_response_with_unicode_data() {
        let json = r#"{
            "app": {
                "app_token": "unicode_token",
                "name": "多维表格📊数据分析🔍",
                "revision": 1,
                "url": "https://飞书.cn/base/unicode_token"
            }
        }"#;

        let response: CreateAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.app.name, "多维表格📊数据分析🔍");
        assert_eq!(response.app.url, "https://飞书.cn/base/unicode_token");
    }

    #[test]
    fn test_create_app_request_body_various_timezones() {
        let timezones = vec![
            "UTC",
            "Asia/Shanghai",
            "America/New_York",
            "Europe/London",
            "Asia/Tokyo",
            "Australia/Sydney",
        ];

        for tz in timezones {
            let body = CreateAppRequestBody {
                name: "时区测试".to_string(),
                folder_token: None,
                time_zone: Some(tz.to_string()),
            };

            let serialized = serde_json::to_value(&body).unwrap();
            let expected = json!({
                "name": "时区测试",
                "time_zone": tz
            });

            assert_eq!(serialized, expected);
        }
    }

    #[test]
    fn test_memory_efficiency() {
        let request = CreateAppRequest::builder().name("内存测试").build();

        let size = std::mem::size_of_val(&request);
        assert!(size > 0);
        assert!(size < 1024);
    }

    #[test]
    fn test_create_app_request_with_long_name() {
        let long_name = "a".repeat(1000);
        let request = CreateAppRequest::builder().name(&long_name).build();

        assert_eq!(request.name, long_name);
    }

    #[test]
    fn test_create_app_request_builder_method_returns() {
        let builder = CreateAppRequest::builder().name("测试链式");

        // 确保builder方法返回正确的类型
        let _chained = builder.folder_token("folder").time_zone("UTC");
    }
}
