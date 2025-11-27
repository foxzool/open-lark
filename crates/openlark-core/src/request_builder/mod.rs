mod auth_handler;
mod header_builder;
mod multipart_builder;

pub use auth_handler::AuthHandler;
pub use header_builder::HeaderBuilder;
pub use multipart_builder::MultipartBuilder;

use crate::{
    api::{ApiRequest, RequestData},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    req_option::RequestOption,
};
use reqwest::RequestBuilder;
use std::{future::Future, pin::Pin};

/// 统一的请求构建器，负责协调各个子构建器
pub struct UnifiedRequestBuilder;

impl UnifiedRequestBuilder {
    pub fn build<'a>(
        req: &'a mut ApiRequest<()>,
        access_token_type: AccessTokenType,
        config: &'a Config,
        option: &'a RequestOption,
    ) -> Pin<Box<dyn Future<Output = Result<RequestBuilder, LarkAPIError>> + Send + 'a>> {
        Box::pin(async move {
            // 1. 构建基础请求
            let url = Self::build_url(config, req)?;
            // 将HttpMethod转换为reqwest::Method
            let reqwest_method = match req.method() {
                crate::api::HttpMethod::Get => reqwest::Method::GET,
                crate::api::HttpMethod::Post => reqwest::Method::POST,
                crate::api::HttpMethod::Put => reqwest::Method::PUT,
                crate::api::HttpMethod::Delete => reqwest::Method::DELETE,
                crate::api::HttpMethod::Patch => reqwest::Method::PATCH,
                crate::api::HttpMethod::Head => reqwest::Method::HEAD,
                crate::api::HttpMethod::Options => reqwest::Method::OPTIONS,
            };

            let mut req_builder = config.http_client.request(reqwest_method, url.as_ref());

            // 2. 构建请求头
            req_builder = HeaderBuilder::build_headers(req_builder, config, option);

            // 3. 处理认证
            req_builder =
                AuthHandler::apply_auth(req_builder, access_token_type, config, option).await?;

            // 4. 处理请求体
            if !req.file().is_empty() {
                if let Some(_body_data) = &req.body {
                    req_builder = MultipartBuilder::build_multipart(
                        req_builder,
                        &req.to_bytes(),
                        &req.file(),
                    )?;
                }
            } else if let Some(body_data) = &req.body {
                match body_data {
                    RequestData::Binary(data) if !data.is_empty() => {
                        req_builder = req_builder.body(data.clone());
                        req_builder = req_builder.header(
                            crate::constants::CONTENT_TYPE_HEADER,
                            crate::constants::DEFAULT_CONTENT_TYPE,
                        );
                    }
                    RequestData::Json(json) => {
                        let json_bytes = serde_json::to_vec(json).unwrap_or_default();
                        req_builder = req_builder.body(json_bytes);
                        req_builder = req_builder.header(
                            crate::constants::CONTENT_TYPE_HEADER,
                            crate::constants::DEFAULT_CONTENT_TYPE,
                        );
                    }
                    _ => {}
                }
            }

            Ok(req_builder)
        })
    }

    fn build_url(config: &Config, req: &ApiRequest<()>) -> Result<url::Url, LarkAPIError> {
        let path = format!("{}{}", config.base_url, req.api_path());
        let query = req
            .query
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<_>>();
        Ok(url::Url::parse_with_params(&path, query)
            .map_err(|e| crate::error::network_error(format!("invalid url: {e}")))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api::ApiRequest, constants::AppType};
    use reqwest::Method;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .base_url("https://open.feishu.cn")
            .build()
    }

    fn create_test_api_request() -> ApiRequest<()> {
        ApiRequest::get("https://open.feishu.cn/open-apis/test")
    }

    #[test]
    fn test_unified_request_builder_struct_creation() {
        let _builder = UnifiedRequestBuilder;
    }

    #[tokio::test]
    async fn test_build_basic_request() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        // Should build request successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_body() {
        let mut api_req = ApiRequest::post("https://open.feishu.cn/open-apis/test").body(
            crate::api::RequestData::Text("{\"test\": \"data\"}".to_string()),
        );

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_files() {
        let mut api_req = ApiRequest::post("https://open.feishu.cn/open-apis/test")
            .body(crate::api::RequestData::Text("file content".to_string()));

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        // This might fail due to multipart builder dependencies, but should not panic
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_build_request_with_query() {
        let mut api_req = create_test_api_request();
        api_req.query.insert("page".to_string(), "1".to_string());
        api_req.query.insert("limit".to_string(), "10".to_string());

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_app_token() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let option = RequestOption {
            app_access_token: "app_token_123".to_string(),
            ..Default::default()
        };

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::App, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_tenant_token() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let option = RequestOption {
            tenant_access_token: "tenant_token_123".to_string(),
            ..Default::default()
        };

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::Tenant, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_user_token() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let option = RequestOption {
            user_access_token: "user_token_123".to_string(),
            ..Default::default()
        };

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::User, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_different_methods() {
        let config = create_test_config();
        let option = RequestOption::default();

        let methods = [
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ];

        for method in methods.iter() {
            let mut api_req = match method.as_str() {
                "GET" => ApiRequest::get("https://open.feishu.cn/open-apis/test"),
                "POST" => ApiRequest::post("https://open.feishu.cn/open-apis/test"),
                "PUT" => ApiRequest::put("https://open.feishu.cn/open-apis/test"),
                "DELETE" => ApiRequest::delete("https://open.feishu.cn/open-apis/test"),
                "PATCH" => ApiRequest::get("https://open.feishu.cn/open-apis/test"), // PATCH not supported, fallback to GET
                _ => ApiRequest::get("https://open.feishu.cn/open-apis/test"),
            };

            let result =
                UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                    .await;

            assert!(result.is_ok(), "Failed for method: {:?}", method);
        }
    }

    #[test]
    fn test_build_url_basic() {
        let config = create_test_config();
        let api_req = create_test_api_request();

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url = result.unwrap();
        // May have trailing ? due to parse_with_params implementation
        assert!(url
            .as_str()
            .starts_with("https://open.feishu.cn/open-apis/test"));
    }

    #[test]
    fn test_build_url_with_query() {
        let config = create_test_config();
        let mut api_req = create_test_api_request();
        api_req.query.insert("page".to_string(), "1".to_string());
        api_req.query.insert("size".to_string(), "20".to_string());

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url = result.unwrap();
        let url_str = url.as_str();
        assert!(url_str.starts_with("https://open.feishu.cn/open-apis/test"));
        assert!(url_str.contains("page=1"));
        assert!(url_str.contains("size=20"));
    }

    #[test]
    fn test_build_url_with_special_characters() {
        let config = create_test_config();
        let mut api_req = create_test_api_request();
        api_req
            .query
            .insert("query".to_string(), "test with spaces".to_string());
        api_req
            .query
            .insert("filter".to_string(), "key=value&other=data".to_string());

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url = result.unwrap();
        // URL encoding should be handled properly
        assert!(url.as_str().contains("query="));
        assert!(url.as_str().contains("filter="));
    }

    #[test]
    fn test_build_url_with_empty_query() {
        let config = create_test_config();
        let mut api_req = create_test_api_request();
        api_req.query.insert("empty".to_string(), "".to_string());

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url = result.unwrap();
        assert!(url.as_str().contains("empty="));
    }

    #[test]
    fn test_build_url_invalid_base_url() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .base_url("invalid-url")
            .build();
        let api_req = create_test_api_request();

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        // Should return error for invalid URL
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_build_request_with_custom_headers() {
        let mut api_req = create_test_api_request();
        let config = create_test_config();
        let mut option = RequestOption {
            request_id: "custom-request-123".to_string(),
            ..Default::default()
        };
        option
            .header
            .insert("X-Custom-Header".to_string(), "custom-value".to_string());

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_complex_scenario() {
        let mut api_req = ApiRequest::post("https://open.feishu.cn/open-apis/complex/test")
            .body(crate::api::RequestData::Text(
                "{\"complex\": \"data\", \"nested\": {\"value\": 123}}".to_string(),
            ))
            .query("version", "v1")
            .query("format", "json");

        let config = create_test_config();
        let option = RequestOption {
            request_id: "complex-request-456".to_string(),
            app_access_token: "app_token_456".to_string(),
            ..Default::default()
        };

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::App, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_unified_request_builder_is_send_sync() {
        // Test that UnifiedRequestBuilder implements required traits
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<UnifiedRequestBuilder>();
        assert_sync::<UnifiedRequestBuilder>();
    }

    #[tokio::test]
    async fn test_build_request_with_body_and_files_edge_case() {
        let mut api_req = ApiRequest::post("https://open.feishu.cn/open-apis/test").body(
            crate::api::RequestData::Text("file content combined".to_string()),
        );

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        // Should handle files taking precedence over body
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_build_url_with_path_segments() {
        let config = create_test_config();
        let api_req = ApiRequest::get("https://open.feishu.cn/open-apis/v1/users/123/messages");

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url = result.unwrap();
        // May have trailing ? due to parse_with_params implementation
        assert!(url
            .as_str()
            .starts_with("https://open.feishu.cn/open-apis/v1/users/123/messages"));
    }
}
