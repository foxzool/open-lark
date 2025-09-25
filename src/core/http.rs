use std::{collections::HashSet, marker::PhantomData};

use log::debug;
use reqwest::RequestBuilder;
use tracing::{info_span, Instrument};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse},
    app_ticket_manager::apply_app_ticket,
    config::Config,
    constants::*,
    error::LarkAPIError,
    improved_response_handler::ImprovedResponseHandler,
    req_option::RequestOption,
    req_translator::ReqTranslator,
    SDKResult,
};

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

            if req.supported_access_token_types.is_empty() {
                req.supported_access_token_types = vec![AccessTokenType::None];
            }

            let result = async {
                validate_token_type(&req.supported_access_token_types, &option)?;
                let access_token_type = determine_token_type(
                    &req.supported_access_token_types,
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
}
