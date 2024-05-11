use std::collections::HashSet;

use log::debug;
use reqwest::blocking::{Request, Response};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::core::api_req::ApiReq;
use crate::core::api_resp::{ApiResp, CodeError};
use crate::core::app_ticket_manager::apply_app_ticket;
use crate::core::config::Config;
use crate::core::constants::{
    AccessTokenType, AppType, CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON,
    ERR_CODE_ACCESS_TOKEN_INVALID, ERR_CODE_APP_ACCESS_TOKEN_INVALID, ERR_CODE_APP_TICKET_INVALID,
    ERR_CODE_TENANT_ACCESS_TOKEN_INVALID, HTTP_HEADER_KEY_REQUEST_ID, HTTP_HEADER_REQUEST_ID,
};
use crate::core::error::LarkAPIError;
use crate::core::req_option::{RequestOption, RequestOptionFunc};
use crate::core::req_translator::ReqTranslator;
use crate::core::SDKResult;

pub struct Transport;

impl Transport {
    pub fn request(
        mut req: ApiReq,
        config: &Config,
        options: Vec<RequestOptionFunc>,
    ) -> Result<ApiResp, LarkAPIError> {
        let mut option = RequestOption::default();

        for option_func in options {
            option_func(&mut option);
        }

        if req.supported_access_token_types.is_empty() {
            req.supported_access_token_types = vec![AccessTokenType::None];
        }

        validate_token_type(&req.supported_access_token_types, &option)?;
        let access_token_type = determine_token_type(
            &req.supported_access_token_types,
            &option,
            config.enable_token_cache,
        );
        validate(&config, &option, access_token_type)?;

        Self::do_request(&req, access_token_type, &config, option)
    }

    fn do_request(
        http_req: &ApiReq,
        access_token_type: AccessTokenType,
        config: &Config,
        option: RequestOption,
    ) -> SDKResult<ApiResp> {
        let mut raw_resp = ApiResp::default();
        for _i in 0..2 {
            let req = ReqTranslator::translate(http_req, access_token_type, config, &option)?;
            debug!("Req:{:?}", req);

            raw_resp = Self::do_send(req, &config.http_client)?;
            debug!("Res:{:?}", raw_resp);

            let file_download_success =
                option.file_upload && raw_resp.status_code == StatusCode::OK;
            if file_download_success
                || raw_resp
                    .header
                    .get(CONTENT_TYPE_HEADER)
                    .is_some_and(|v| v.to_str().unwrap().contains(CONTENT_TYPE_JSON))
            {
                break;
            }

            let code_error: CodeError = serde_json::from_slice(&raw_resp.raw_body.clone())?;
            let code = code_error.code;
            if code == ERR_CODE_APP_TICKET_INVALID {
                apply_app_ticket(config)?;
            }
            if access_token_type == AccessTokenType::None {
                break;
            }
            if !config.enable_token_cache {
                break;
            }

            if code != ERR_CODE_ACCESS_TOKEN_INVALID
                && code != ERR_CODE_APP_ACCESS_TOKEN_INVALID
                && code != ERR_CODE_TENANT_ACCESS_TOKEN_INVALID
            {
                break;
            }
        }

        Ok(raw_resp)
    }

    fn do_send(raw_request: Request, client: &reqwest::blocking::Client) -> SDKResult<ApiResp> {
        let response = client.execute(raw_request)?;
        Ok(ApiResp {
            status_code: response.status().as_u16(),
            header: response.headers().clone(),
            raw_body: response.bytes()?,
        })
    }
}

fn validate_token_type(
    access_token_types: &[AccessTokenType],
    option: &RequestOption,
) -> Result<(), LarkAPIError> {
    if !access_token_types.is_empty() {
        return Ok(());
    }

    let access_token_type = access_token_types[0].clone();

    if access_token_type == AccessTokenType::Tenant {
        if !option.user_access_token.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "tenant token type not match user access token".to_string(),
            ));
        }
    }

    if access_token_type == AccessTokenType::App {
        if !option.tenant_access_token.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "user token type not match tenant access token".to_string(),
            ));
        }
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

    if !option.tenant_key.is_empty() {
        if accessible_token_type_set.contains(&AccessTokenType::Tenant) {
            access_token_type = AccessTokenType::Tenant;
        }
    }

    if !option.user_access_token.is_empty() {
        if accessible_token_type_set.contains(&AccessTokenType::User) {
            access_token_type = AccessTokenType::User;
        }
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

    if option.header.get(HTTP_HEADER_KEY_REQUEST_ID).is_some() {
        return Err(LarkAPIError::IllegalParamError(format!(
            "use {} as header key is not allowed",
            HTTP_HEADER_KEY_REQUEST_ID
        )));
    }
    if option.header.get(HTTP_HEADER_REQUEST_ID).is_some() {
        return Err(LarkAPIError::IllegalParamError(format!(
            "use {} as header key is not allowed",
            HTTP_HEADER_REQUEST_ID
        )));
    }

    Ok(())
}
