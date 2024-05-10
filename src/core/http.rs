use std::collections::{HashMap, HashSet};

use log::debug;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;

use crate::core::api_req::ApiReq;
use crate::core::config::Config;
use crate::core::constants::{
    AccessTokenType, AppType, AUTHORIZATION, HTTP_HEADER_KEY_REQUEST_ID, HTTP_HEADER_REQUEST_ID,
    PROJECT, USER_AGENT, VERSION,
};
use crate::core::error::LarkAPIError;
use crate::core::model::{
    BaseRequest, BaseResponse, BaseResponseTrait, RawResponse, RequestOption as OpOld,
};
use crate::core::req_option::{RequestOption, RequestOptionFunc};

pub struct Transport;

impl Transport {
    pub fn request(
        mut req: ApiReq,
        config: Config,
        options: Vec<RequestOptionFunc>,
    ) -> Result<(), LarkAPIError> {
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

        Ok(())
    }
    pub fn execute(
        config: &Config,
        req: &BaseRequest,
        option: Option<OpOld>,
    ) -> Result<Response, LarkAPIError> {
        let option = option.unwrap_or_default();

        // 拼接url
        let url = build_url(
            &config.base_url,
            &req.uri.clone().unwrap_or_default(),
            &req.paths,
        );

        // 组装header
        let headers = build_header(&req.clone(), &option);
        let data = req.body.clone().unwrap();

        let client = reqwest::blocking::Client::new();
        let response = client
            .request(req.http_method.clone().unwrap_or_default(), &url)
            .headers(headers.clone())
            .body(data.to_string())
            .send()?;

        let queries_json = serde_json::to_string(&req.queries)?;

        debug!(
            "{} {} {}, headers: {:?}, queries: {:?}, body: {:?} ",
            req.http_method.clone().unwrap_or_default(),
            url,
            response.status(),
            headers,
            queries_json,
            data
        );

        println!("{:?}", response);

        Ok(response)
    }
}

fn build_url(domain: &str, uri: &str, paths: &HashMap<String, String>) -> String {
    let mut uri = uri.to_string();
    for (key, value) in paths {
        uri = uri.replace(&(":".to_string() + key), value);
    }

    domain.to_string() + &uri
}

fn build_header(request: &BaseRequest, option: &OpOld) -> HeaderMap {
    let mut headers = HeaderMap::try_from(&request.headers).unwrap_or_default();

    // 添加ua
    headers.insert(
        USER_AGENT,
        format!("{}/v{}", PROJECT, VERSION).parse().unwrap(),
    );

    let opt_headers: HeaderMap<String> = HeaderMap::try_from(&option.headers).unwrap_or_default();

    // 附加header
    for (key, value) in opt_headers {
        headers.insert(key.unwrap(), value.parse().unwrap());
    }

    // 添加token
    for token_type in &request.token_types {
        match token_type {
            AccessTokenType::Tenant => {
                headers.insert(
                    AUTHORIZATION,
                    format!("Bearer {}", option.tenant_access_token.as_ref().unwrap())
                        .parse()
                        .unwrap(),
                );
            }
            AccessTokenType::App => {
                headers.insert(
                    AUTHORIZATION,
                    format!("Bearer {}", option.app_access_token.as_ref().unwrap())
                        .parse()
                        .unwrap(),
                );
            }
            AccessTokenType::User => {
                headers.insert(
                    AUTHORIZATION,
                    format!("Bearer {}", option.user_access_token.as_ref().unwrap())
                        .parse()
                        .unwrap(),
                );
            }
            _ => {}
        }
    }

    headers
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
    if config.app_id.is_none() {
        return Err(LarkAPIError::IllegalParamError(
            "AppId is empty".to_string(),
        ));
    }

    if config.app_secret.is_none() {
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
