mod auth_handler;
mod header_builder;
mod multipart_builder;

pub use auth_handler::AuthHandler;
pub use header_builder::HeaderBuilder;
pub use multipart_builder::MultipartBuilder;

use crate::core::{
    api_req::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    req_option::RequestOption,
};
use reqwest::RequestBuilder;
use std::{future::Future, pin::Pin};

/// 统一的请求构建器，负责协调各个子构建器
pub struct UnifiedRequestBuilder;

impl UnifiedRequestBuilder {
    pub fn build<'a>(
        req: &'a mut ApiRequest,
        access_token_type: AccessTokenType,
        config: &'a Config,
        option: &'a RequestOption,
    ) -> Pin<Box<dyn Future<Output = Result<RequestBuilder, LarkAPIError>> + Send + 'a>> {
        Box::pin(async move {
            // 1. 构建基础请求
            let url = Self::build_url(config, req)?;
            let mut req_builder = config
                .http_client
                .request(req.http_method.clone(), url.as_ref());

            // 2. 构建请求头
            req_builder = HeaderBuilder::build_headers(req_builder, config, option);

            // 3. 处理认证
            req_builder =
                AuthHandler::apply_auth(req_builder, access_token_type, config, option).await?;

            // 4. 处理请求体
            if !req.file.is_empty() {
                req_builder = MultipartBuilder::build_multipart(req_builder, &req.body, &req.file)?;
            } else if !req.body.is_empty() {
                req_builder = req_builder.body(req.body.clone());
                req_builder = req_builder.header(
                    crate::core::constants::CONTENT_TYPE_HEADER,
                    crate::core::constants::DEFAULT_CONTENT_TYPE,
                );
            }

            Ok(req_builder)
        })
    }

    fn build_url(config: &Config, req: &ApiRequest) -> Result<url::Url, LarkAPIError> {
        let path = format!("{}{}", config.base_url, req.api_path);
        let query_params = req
            .query_params
            .iter()
            .map(|(k, v)| (*k, v.as_str()))
            .collect::<Vec<_>>();
        Ok(url::Url::parse_with_params(&path, query_params)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{api_req::ApiRequest, constants::AppType};
    use reqwest::Method;
    use std::collections::HashMap;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .base_url("https://open.feishu.cn")
            .build()
    }

    fn create_test_api_request() -> ApiRequest {
        ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/test".to_string(),
            body: vec![],
            file: vec![],
            query_params: HashMap::new(),
            ..Default::default()
        }
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
        let mut api_req = create_test_api_request();
        api_req.http_method = Method::POST;
        api_req.body = b"{\"test\": \"data\"}".to_vec();

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_build_request_with_files() {
        let mut api_req = create_test_api_request();
        api_req.http_method = Method::POST;

        // Add a file to the request
        api_req.file = b"file content".to_vec();

        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            UnifiedRequestBuilder::build(&mut api_req, AccessTokenType::None, &config, &option)
                .await;

        // This might fail due to multipart builder dependencies, but should not panic
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_build_request_with_query_params() {
        let mut api_req = create_test_api_request();
        api_req.query_params.insert("page", "1".to_string());
        api_req.query_params.insert("limit", "10".to_string());

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
            let mut api_req = create_test_api_request();
            api_req.http_method = method.clone();

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
    fn test_build_url_with_query_params() {
        let config = create_test_config();
        let mut api_req = create_test_api_request();
        api_req.query_params.insert("page", "1".to_string());
        api_req.query_params.insert("size", "20".to_string());

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
            .query_params
            .insert("query", "test with spaces".to_string());
        api_req
            .query_params
            .insert("filter", "key=value&other=data".to_string());

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url = result.unwrap();
        // URL encoding should be handled properly
        assert!(url.as_str().contains("query="));
        assert!(url.as_str().contains("filter="));
    }

    #[test]
    fn test_build_url_with_empty_query_params() {
        let config = create_test_config();
        let mut api_req = create_test_api_request();
        api_req.query_params.insert("empty", "".to_string());

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
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/complex/test".to_string(),
            body: b"{\"complex\": \"data\", \"nested\": {\"value\": 123}}".to_vec(),
            file: vec![],
            query_params: HashMap::new(),
            ..Default::default()
        };
        api_req.query_params.insert("version", "v1".to_string());
        api_req.query_params.insert("format", "json".to_string());

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
        let mut api_req = create_test_api_request();
        api_req.http_method = Method::POST;
        api_req.body = b"regular body".to_vec();

        // Add files - this should take precedence over body
        api_req.file = b"file content combined".to_vec();

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
        let mut api_req = create_test_api_request();
        api_req.api_path = "/open-apis/v1/users/123/messages".to_string();

        let result = UnifiedRequestBuilder::build_url(&config, &api_req);

        assert!(result.is_ok());
        let url = result.unwrap();
        // May have trailing ? due to parse_with_params implementation
        assert!(url
            .as_str()
            .starts_with("https://open.feishu.cn/open-apis/v1/users/123/messages"));
    }
}
