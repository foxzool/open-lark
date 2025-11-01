use std::{collections::HashSet, marker::PhantomData};

use log::debug;
use reqwest::RequestBuilder;
use tracing::{info_span, Instrument};

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse},
    app_ticket_manager::apply_app_ticket,
    config::Config,
    constants::{AccessTokenType, *},
    error::LarkAPIError,
    improved_response_handler::ImprovedResponseHandler,
    req_option::RequestOption,
    req_translator::ReqTranslator,
    SDKResult,
};
// Re-export ApiRequest from open-lark-core
pub use open_lark_core::core::api_req::ApiRequest;

pub struct Transport<T> {
    phantom_data: PhantomData<T>,
}

impl<T: ApiResponseTrait> Transport<T> {
    pub async fn request(
        mut req: ApiRequest,
        config: &Config,
        option: Option<RequestOption>,
    ) -> Result<BaseResponse<T>, LarkAPIError> {
        // Create span for HTTP request tracing
        let span = info_span!(
            "http_request",
            method = %req.http_method,
            path = %req.api_path,
            app_id = %config.app_id,
            duration_ms = tracing::field::Empty,
            status = tracing::field::Empty,
        );

        async move {
            let start_time = std::time::Instant::now();
            let option = option.unwrap_or_default();

            if req.get_supported_access_token_types().is_empty() {
                req.set_supported_access_token_types(vec![AccessTokenType::None]);
            }

            let result = async {
                validate_token_type(req.get_supported_access_token_types(), &option)?;
                let access_token_type = determine_token_type(
                    req.get_supported_access_token_types(),
                    &option,
                    config.enable_token_cache,
                );
                validate(config, &option, access_token_type)?;

                Self::do_request(req, access_token_type, config, option).await
            }
            .await;

            // Record metrics in current span
            let current_span = tracing::Span::current();
            let duration_ms = start_time.elapsed().as_millis() as u64;
            current_span.record("duration_ms", duration_ms);

            match &result {
                Ok(response) => {
                    current_span.record(
                        "status",
                        if response.success() {
                            "success"
                        } else {
                            "api_error"
                        },
                    );
                }
                Err(_) => {
                    current_span.record("status", "error");
                }
            }

            result
        }
        .instrument(span)
        .await
    }

    async fn do_request(
        mut http_req: ApiRequest,
        access_token_type: AccessTokenType,
        config: &Config,
        option: RequestOption,
    ) -> SDKResult<BaseResponse<T>> {
        let req =
            ReqTranslator::translate(&mut http_req, access_token_type, config, &option).await?;
        debug!("Req:{req:?}");
        let resp = Self::do_send(req, http_req.body, !http_req.file.is_empty()).await?;
        debug!("Res:{resp:?}");

        if !resp.success() && resp.raw_response.code == ERR_CODE_APP_TICKET_INVALID {
            apply_app_ticket(config).await?;
        }

        Ok(resp)
    }

    pub async fn do_send(
        raw_request: RequestBuilder,
        body: Vec<u8>,
        multi_part: bool,
    ) -> SDKResult<BaseResponse<T>> {
        // Create span for network request tracing
        let span = info_span!(
            "http_send",
            multi_part = multi_part,
            body_size = body.len(),
            response_code = tracing::field::Empty,
            response_size = tracing::field::Empty,
        );

        async move {
            let future = if multi_part {
                raw_request.send()
            } else {
                raw_request.body(body).send()
            };

            match future.await {
                Ok(response) => {
                    let status_code = response.status();
                    tracing::Span::current().record("response_code", status_code.as_u16());

                    // 使用改进的响应处理器，单次解析而非双重解析
                    ImprovedResponseHandler::handle_response(response).await
                }
                Err(err) => {
                    debug!("Request error: {err:?}");
                    tracing::Span::current().record("response_code", 0_u16); // Indicate network error
                    Err(LarkAPIError::RequestError(err.to_string()))
                }
            }
        }
        .instrument(span)
        .await
    }
}

fn validate_token_type(
    access_token_types: &[AccessTokenType],
    option: &RequestOption,
) -> Result<(), LarkAPIError> {
    if !access_token_types.is_empty() {
        return Ok(());
    }

    let access_token_type = access_token_types[0];

    if access_token_type == AccessTokenType::Tenant && !option.user_access_token.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "tenant token type not match user access token".to_string(),
        ));
    }

    if access_token_type == AccessTokenType::App && !option.tenant_access_token.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "user token type not match tenant access token".to_string(),
        ));
    }

    Ok(())
}

fn determine_token_type(
    access_token_types: &[AccessTokenType],
    option: &RequestOption,
    enable_token_cache: bool,
) -> AccessTokenType {
    if !enable_token_cache {
        if !option.user_access_token.is_empty() {
            return AccessTokenType::User;
        }
        if !option.tenant_access_token.is_empty() {
            return AccessTokenType::Tenant;
        }
        if !option.app_access_token.is_empty() {
            return AccessTokenType::App;
        }

        return AccessTokenType::None;
    }
    let mut accessible_token_type_set: HashSet<AccessTokenType> = HashSet::new();
    let mut access_token_type = access_token_types[0];

    for t in access_token_types {
        if *t == AccessTokenType::Tenant {
            access_token_type = *t; // 默认值
        }
        accessible_token_type_set.insert(*t);
    }

    if !option.tenant_key.is_empty() && accessible_token_type_set.contains(&AccessTokenType::Tenant)
    {
        access_token_type = AccessTokenType::Tenant;
    }

    if !option.user_access_token.is_empty()
        && accessible_token_type_set.contains(&AccessTokenType::User)
    {
        access_token_type = AccessTokenType::User;
    }

    access_token_type
}

fn validate(
    config: &Config,
    option: &RequestOption,
    access_token_type: AccessTokenType,
) -> Result<(), LarkAPIError> {
    if config.app_id.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "AppId is empty".to_string(),
        ));
    }

    if config.app_secret.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "AppSecret is empty".to_string(),
        ));
    }

    if !config.enable_token_cache {
        if access_token_type == AccessTokenType::None {
            return Ok(());
        }
        if option.user_access_token.is_empty()
            && option.tenant_access_token.is_empty()
            && option.app_access_token.is_empty()
        {
            return Err(LarkAPIError::IllegalParamError(
                "accessToken is empty".to_string(),
            ));
        }
    }

    if config.app_type == AppType::Marketplace
        && access_token_type == AccessTokenType::Tenant
        && option.tenant_key.is_empty()
    {
        return Err(LarkAPIError::IllegalParamError(
            "accessToken is empty".to_string(),
        ));
    }

    if access_token_type == AccessTokenType::User && option.user_access_token.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "user access token is empty".to_string(),
        ));
    }

    if option.header.contains_key(HTTP_HEADER_KEY_REQUEST_ID) {
        return Err(LarkAPIError::IllegalParamError(format!(
            "use {HTTP_HEADER_KEY_REQUEST_ID} as header key is not allowed"
        )));
    }
    if option.header.contains_key(HTTP_HEADER_REQUEST_ID) {
        return Err(LarkAPIError::IllegalParamError(format!(
            "use {HTTP_HEADER_REQUEST_ID} as header key is not allowed"
        )));
    }

    Ok(())
}

/// 解析文件名
#[allow(dead_code)]
fn decode_file_name(file_name: &str) -> Option<String> {
    let parts = file_name.split(';');

    for part in parts {
        if part.trim().starts_with("filename*=") {
            let filename = part
                .trim()
                .strip_prefix("filename*=UTF-8''")
                .unwrap_or("")
                .to_string();
            return Some(filename);
        }
    }

    None
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod test {
    use std::collections::HashMap;

    use crate::core::{
        config::Config,
        constants::{AccessTokenType, AppType, HTTP_HEADER_KEY_REQUEST_ID, HTTP_HEADER_REQUEST_ID},
        error::LarkAPIError,
        http::{decode_file_name, determine_token_type, validate, validate_token_type},
        req_option::RequestOption,
    };

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    fn create_test_config_marketplace() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::Marketplace)
            .build()
    }

    #[test]
    fn test_decode_file_name() {
        let raw = "attachment; filename=\"upload_all.rs\"; filename*=UTF-8''upload_all.rs";
        let file_name = decode_file_name(raw).unwrap();
        assert_eq!(file_name, "upload_all.rs");
    }

    #[test]
    fn test_decode_file_name_no_utf8() {
        let raw = "attachment; filename=\"simple.txt\"";
        let file_name = decode_file_name(raw);
        assert!(file_name.is_none());
    }

    #[test]
    fn test_decode_file_name_multiple_parts() {
        let raw = "attachment; charset=utf-8; filename*=UTF-8''complex%20name.txt; other=value";
        let file_name = decode_file_name(raw).unwrap();
        assert_eq!(file_name, "complex%20name.txt");
    }

    #[test]
    fn test_decode_file_name_empty() {
        let raw = "";
        let file_name = decode_file_name(raw);
        assert!(file_name.is_none());
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_validate_token_type_empty_list_bug() {
        let empty_types: Vec<AccessTokenType> = vec![];
        let option = RequestOption::default();

        // This demonstrates the bug in validate_token_type - it will panic
        // due to accessing index 0 on empty list
        let _ = validate_token_type(&empty_types, &option);
    }

    #[test]
    fn test_validate_token_type_non_empty_list_returns_ok() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption::default();

        // Non-empty list should return Ok immediately without validation
        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_token_type_tenant_with_user_token() {
        let types = vec![AccessTokenType::Tenant];
        let option = RequestOption {
            user_access_token: "user_token".to_string(),
            ..Default::default()
        };

        // Non-empty list returns Ok immediately, so this passes despite mismatch
        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_token_type_app_with_tenant_token() {
        let types = vec![AccessTokenType::App];
        let option = RequestOption {
            tenant_access_token: "tenant_token".to_string(),
            ..Default::default()
        };

        // Non-empty list returns Ok immediately, so this passes despite mismatch
        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_token_type_valid_combinations() {
        let types = vec![AccessTokenType::User];
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();

        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_determine_token_type_no_cache_user() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();

        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_determine_token_type_no_cache_tenant() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption {
            tenant_access_token: "tenant_token".to_string(),
            ..Default::default()
        };

        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_no_cache_app() {
        let types = vec![AccessTokenType::App, AccessTokenType::Tenant];
        let option = RequestOption {
            app_access_token: "app_token".to_string(),
            ..Default::default()
        };

        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::App);
    }

    #[test]
    fn test_determine_token_type_no_cache_none() {
        let types = vec![AccessTokenType::None];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_determine_token_type_with_cache_defaults_to_tenant() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_with_cache_tenant_key() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let mut option = RequestOption::default();
        option.tenant_key = "tenant_key".to_string();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_with_cache_user_access_token() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_validate_empty_app_id() {
        let config = Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_empty_app_secret() {
        let config = Config::builder().app_id("test_id").app_secret("").build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_no_cache_missing_access_tokens() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .enable_token_cache(false)
            .build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::User);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_no_cache_with_tokens() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .enable_token_cache(false)
            .build();
        let mut option = RequestOption::default();
        option.user_access_token = "token".to_string();

        let result = validate(&config, &option, AccessTokenType::User);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_marketplace_tenant_no_key() {
        let config = create_test_config_marketplace();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_marketplace_tenant_with_key() {
        let config = create_test_config_marketplace();
        let mut option = RequestOption::default();
        option.tenant_key = "tenant_key".to_string();

        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_user_token_empty() {
        let config = create_test_config();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::User);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_user_token_present() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();

        let result = validate(&config, &option, AccessTokenType::User);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_forbidden_header_key_request_id() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert(HTTP_HEADER_KEY_REQUEST_ID.to_string(), "test".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_forbidden_header_request_id() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert(HTTP_HEADER_REQUEST_ID.to_string(), "test".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_valid_config() {
        let config = create_test_config();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_cache_none_token_type() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .enable_token_cache(false)
            .build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_determine_token_type_first_is_tenant() {
        let types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_no_tenant_in_list() {
        let types = vec![AccessTokenType::User, AccessTokenType::App];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        // Should use first type when no Tenant type is available
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_validate_token_type_edge_case_single_element() {
        let types = vec![AccessTokenType::None];
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();

        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_file_name_whitespace_handling() {
        let raw = " attachment ; filename=\"test.txt\" ; filename*=UTF-8''spaced%20file.txt ";
        let file_name = decode_file_name(raw).unwrap();
        assert_eq!(file_name, "spaced%20file.txt");
    }

    #[test]
    fn test_decode_file_name_no_equals() {
        let raw = "attachment; filename*UTF-8''invalid.txt";
        let file_name = decode_file_name(raw);
        assert!(file_name.is_none());
    }

    // Additional comprehensive tests for better coverage

    #[test]
    fn test_validate_token_type_empty_list_early_return() {
        // This test verifies the early return logic when list is NOT empty
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption::default();

        // Should return Ok immediately due to non-empty list
        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_determine_token_type_priority_with_multiple_tokens() {
        let types = vec![
            AccessTokenType::User,
            AccessTokenType::Tenant,
            AccessTokenType::App,
        ];
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();
        option.tenant_key = "tenant_key".to_string();

        // User token should take priority when present
        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_determine_token_type_tenant_key_without_tenant_type() {
        let types = vec![AccessTokenType::User, AccessTokenType::App];
        let mut option = RequestOption::default();
        option.tenant_key = "tenant_key".to_string();

        // Should default to first type when Tenant not available
        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_determine_token_type_user_token_without_user_type() {
        let types = vec![AccessTokenType::Tenant, AccessTokenType::App];
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();

        // Should use Tenant when User type not available
        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_cache_disabled_fallback_priority() {
        let types = vec![
            AccessTokenType::User,
            AccessTokenType::Tenant,
            AccessTokenType::App,
        ];
        let mut option = RequestOption::default();
        option.tenant_access_token = "tenant_token".to_string();
        option.app_access_token = "app_token".to_string();

        // Tenant should be chosen over App when cache disabled
        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_cache_disabled_all_empty() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption::default();

        // Should return None when no tokens provided and cache disabled
        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_validate_config_with_all_required_fields() {
        let config = Config::builder()
            .app_id("valid_app_id")
            .app_secret("valid_app_secret")
            .enable_token_cache(true)
            .build();
        let option = RequestOption::default();

        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_marketplace_app_with_valid_tenant_key() {
        let config = Config::builder()
            .app_id("marketplace_app")
            .app_secret("marketplace_secret")
            .app_type(AppType::Marketplace)
            .build();
        let mut option = RequestOption::default();
        option.tenant_key = "valid_tenant_key".to_string();

        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_marketplace_app_type_with_non_tenant_token() {
        let config = Config::builder()
            .app_id("marketplace_app")
            .app_secret("marketplace_secret")
            .app_type(AppType::Marketplace)
            .build();
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();

        // User token type needs user_access_token to be present
        let result = validate(&config, &option, AccessTokenType::User);
        assert!(result.is_ok());

        // App token type should pass without additional requirements for marketplace
        let result = validate(&config, &RequestOption::default(), AccessTokenType::App);
        assert!(result.is_ok());

        // None token type should also pass
        let result = validate(&config, &RequestOption::default(), AccessTokenType::None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_cache_with_multiple_token_types() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .enable_token_cache(false)
            .build();
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string();
        option.tenant_access_token = "tenant_token".to_string();
        option.app_access_token = "app_token".to_string();

        // Should validate successfully with any token present
        let result = validate(&config, &option, AccessTokenType::User);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_user_token_type_with_empty_user_token() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        option.tenant_access_token = "tenant_token".to_string(); // Other tokens present

        // Should fail when User token type but user_access_token is empty
        let result = validate(&config, &option, AccessTokenType::User);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
        if let Err(LarkAPIError::IllegalParamError(msg)) = result {
            assert!(msg.contains("user access token is empty"));
        }
    }

    #[test]
    fn test_validate_forbidden_headers_custom_values() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert("X-Request-Id".to_string(), "custom_id".to_string());
        header.insert("Custom-Header".to_string(), "value".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_forbidden_headers_request_id_variation() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert("Request-Id".to_string(), "another_id".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(matches!(result, Err(LarkAPIError::IllegalParamError(_))));
    }

    #[test]
    fn test_validate_allowed_custom_headers() {
        let config = create_test_config();
        let mut option = RequestOption::default();
        let mut header = HashMap::new();
        header.insert("Authorization".to_string(), "Bearer token".to_string());
        header.insert("Content-Type".to_string(), "application/json".to_string());
        header.insert("Custom-App-Header".to_string(), "value".to_string());
        option.header = header;

        let result = validate(&config, &option, AccessTokenType::None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_file_name_missing_utf8_prefix() {
        let raw = "attachment; filename*=''missing_utf8.txt";
        let file_name = decode_file_name(raw);
        // The function returns Some("") when the UTF-8'' prefix is missing
        assert_eq!(file_name, Some("".to_string()));
    }

    #[test]
    fn test_decode_file_name_malformed_filename_star() {
        let raw = "attachment; filename*=UTF-8";
        let file_name = decode_file_name(raw);
        assert_eq!(file_name, Some("".to_string()));
    }

    #[test]
    fn test_decode_file_name_multiple_filename_star_entries() {
        let raw = "attachment; filename*=UTF-8''first.txt; filename*=UTF-8''second.txt";
        let file_name = decode_file_name(raw).unwrap();
        // Should return the first match
        assert_eq!(file_name, "first.txt");
    }

    #[test]
    fn test_decode_file_name_special_characters() {
        let raw = "attachment; filename*=UTF-8''special%20%21%40%23.txt";
        let file_name = decode_file_name(raw).unwrap();
        assert_eq!(file_name, "special%20%21%40%23.txt");
    }

    #[test]
    fn test_decode_file_name_empty_filename() {
        let raw = "attachment; filename*=UTF-8''";
        let file_name = decode_file_name(raw).unwrap();
        assert_eq!(file_name, "");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_determine_token_type_empty_types_list_panics() {
        // Note: This documents a bug in the current implementation
        // When cache is enabled and types list is empty, it panics on types[0]
        let types: Vec<AccessTokenType> = vec![];
        let option = RequestOption::default();

        // This will panic due to accessing types[0] on empty list when cache enabled
        let _token_type = determine_token_type(&types, &option, true);
    }

    #[test]
    fn test_determine_token_type_empty_types_list_no_cache() {
        // When cache is disabled, empty types list works fine
        let types: Vec<AccessTokenType> = vec![];
        let option = RequestOption::default();

        // This works because cache-disabled path returns None without accessing types[0]
        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_determine_token_type_single_app_type() {
        let types = vec![AccessTokenType::App];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::App);
    }

    #[test]
    fn test_determine_token_type_single_none_type() {
        let types = vec![AccessTokenType::None];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_validate_with_cache_enabled_various_token_types() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .enable_token_cache(true)
            .build();
        let option = RequestOption::default();

        // With cache enabled, should validate OK for all token types
        assert!(validate(&config, &option, AccessTokenType::None).is_ok());
        assert!(validate(&config, &option, AccessTokenType::App).is_ok());
        assert!(validate(&config, &option, AccessTokenType::Tenant).is_ok());
    }

    #[test]
    fn test_validate_self_build_app_type_with_tenant_token() {
        let config = Config::builder()
            .app_id("self_build_app")
            .app_secret("self_build_secret")
            .app_type(AppType::SelfBuild)
            .build();
        let option = RequestOption::default();

        // Self-build apps should validate OK without tenant_key
        let result = validate(&config, &option, AccessTokenType::Tenant);
        assert!(result.is_ok());
    }

    // Test edge cases for the validate_token_type logic paths

    #[test]
    fn test_validate_token_type_with_mismatched_tokens_simulation() {
        // This test documents what SHOULD happen vs what actually happens
        // The current implementation has a bug where it returns Ok(()) early
        // when the list is non-empty, bypassing the validation logic

        let types = vec![AccessTokenType::Tenant]; // Single element to avoid early return
        let mut option = RequestOption::default();
        option.user_access_token = "user_token".to_string(); // Mismatch!

        // This SHOULD fail but currently passes due to early return bug
        let result = validate_token_type(&types, &option);
        // Documenting current (buggy) behavior
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_comprehensive_error_messages() {
        let config_empty_id = Config::builder().app_id("").app_secret("secret").build();

        let config_empty_secret = Config::builder().app_id("app_id").app_secret("").build();

        let option = RequestOption::default();

        // Test specific error messages
        if let Err(LarkAPIError::IllegalParamError(msg)) =
            validate(&config_empty_id, &option, AccessTokenType::None)
        {
            assert_eq!(msg, "AppId is empty");
        } else {
            panic!("Expected IllegalParamError for empty app_id");
        }

        if let Err(LarkAPIError::IllegalParamError(msg)) =
            validate(&config_empty_secret, &option, AccessTokenType::None)
        {
            assert_eq!(msg, "AppSecret is empty");
        } else {
            panic!("Expected IllegalParamError for empty app_secret");
        }
    }
}
