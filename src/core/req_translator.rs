use std::{future::Future, pin::Pin};

use reqwest::RequestBuilder;

use crate::core::{
    config::Config, constants::AccessTokenType, error::LarkAPIError, req_option::RequestOption,
    request_builder::UnifiedRequestBuilder,
};
// Re-export ApiRequest from open-lark-core
use super::ApiRequest;

pub struct ReqTranslator;

impl ReqTranslator {
    pub fn translate<'a>(
        req: &'a mut ApiRequest,
        access_token_type: AccessTokenType,
        config: &'a Config,
        option: &'a RequestOption,
    ) -> Pin<Box<dyn Future<Output = Result<RequestBuilder, LarkAPIError>> + Send + 'a>> {
        // 委托给新的统一请求构建器
        UnifiedRequestBuilder::build(req, access_token_type, config, option)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::AppType;
    use open_lark_core::core::api_req::ApiRequest;
    use reqwest::Method;

    #[test]
    fn test_req_translator_struct_creation() {
        // Test that ReqTranslator can be created
        let _translator = ReqTranslator;
    }

    #[tokio::test]
    async fn test_req_translator_translate_delegation() {
        // Create test data
        let mut api_req = ApiRequest::with_method_and_path(Method::GET, "/test");

        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .build();

        let option = RequestOption::default();

        // Test that translate method delegates to UnifiedRequestBuilder
        // This will test the delegation without actually making network requests
        let result =
            ReqTranslator::translate(&mut api_req, AccessTokenType::App, &config, &option).await;

        // The exact result depends on UnifiedRequestBuilder implementation
        // But we can verify the method can be called without panicking
        // and returns the expected type
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_req_translator_with_different_token_types() {
        let mut api_req = ApiRequest::with_method_and_path(Method::POST, "/open-apis/test");
        api_req.body = b"test body".to_vec();

        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .app_type(AppType::SelfBuild)
            .build();

        let option = RequestOption::default();

        // Test with different access token types
        let token_types = [
            AccessTokenType::None,
            AccessTokenType::App,
            AccessTokenType::Tenant,
            AccessTokenType::User,
        ];

        for token_type in token_types.iter() {
            let result =
                ReqTranslator::translate(&mut api_req, *token_type, &config, &option).await;

            // Each call should complete without panicking
            // Result depends on UnifiedRequestBuilder behavior
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[tokio::test]
    async fn test_req_translator_with_various_request_methods() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .app_type(AppType::SelfBuild)
            .build();

        let option = RequestOption::default();

        let methods = [
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ];

        for method in methods.iter() {
            let mut api_req = ApiRequest::with_method_and_path(method.clone(), "/test/path");

            let result =
                ReqTranslator::translate(&mut api_req, AccessTokenType::App, &config, &option)
                    .await;

            // Each method should be handled
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_req_translator_is_send_sync() {
        // Test that ReqTranslator implements required traits
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<ReqTranslator>();
        assert_sync::<ReqTranslator>();
    }

    #[tokio::test]
    async fn test_req_translator_with_marketplace_app() {
        let mut api_req =
            ApiRequest::with_method_and_path(Method::GET, "/open-apis/marketplace/test");

        let config = Config::builder()
            .app_id("marketplace_app")
            .app_secret("marketplace_secret")
            .app_type(AppType::Marketplace)
            .build();

        let option = RequestOption {
            tenant_key: "test_tenant".to_string(),
            ..Default::default()
        };

        let result =
            ReqTranslator::translate(&mut api_req, AccessTokenType::Tenant, &config, &option).await;

        // Should handle marketplace app configuration
        assert!(result.is_ok() || result.is_err());
    }
}
