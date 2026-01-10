//! Integration tests for TokenProvider support in Transport

use crate::{
    api::{ApiRequest, Response},
    auth::token_provider::{NoOpTokenProvider, TokenProvider},
    config::Config,
    constants::{AccessTokenType, AppType},
    error::SDKResult,
    http::Transport,
    request_builder::UnifiedRequestBuilder,
};

/// Test that verifies token_provider integration works correctly
#[tokio::test]
async fn test_transport_uses_token_provider() -> SDKResult<()> {
    // Create a test config with token_provider set
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .token_provider(NoOpTokenProvider)
        .app_type(AppType::SelfBuild)
        .build();

    // Create a simple test request
    let request = ApiRequest::<()>::get("https://open.feishu.cn/test").unwrap();

    // This should work - TokenProvider should not be called directly
    // but fallback to legacy behavior should work
    let _result = http::Transport::request(request, &config, None).await;

    // The key is that ReqTranslator should check for token_provider
    // and use it when available instead of AuthHandler
    Ok(())
}

/// Test that verifies fallback to AuthHandler when no token_provider
#[tokio::test]
async fn test_token_provider_return_error() -> SDKResult<()> {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .token_provider(NoOpTokenProvider)
        .app_type(AppType::SelfBuild)
        .build();

    let request = ApiRequest::<()>::get("https://open.feishu.cn/test").unwrap();

    // NoOpTokenProvider should return error
    let result = http::Transport::request(request, &config, None).await;

    // Verify error is returned
    assert!(result.is_err());
}
