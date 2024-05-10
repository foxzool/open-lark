use std::collections::HashMap;

use log::debug;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;

use crate::core::api_req::ApiReq;
use crate::core::config::Config;
use crate::core::constants::{AccessTokenType, AUTHORIZATION, PROJECT, USER_AGENT, VERSION};
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

        validate_token_type(&req.supported_access_token_types, option)?;

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
    option: RequestOption,
) -> Result<(), LarkAPIError> {
    if !access_token_types.is_empty() {
        return Ok(());
    }

    let access_token_type = access_token_types[0].clone();

    if access_token_type == AccessTokenType::Tenant {
        if !option.user_access_token.is_empty() {
            return Err(LarkAPIError::Custom(
                "tenant token type not match user access token".to_string(),
            ));
        }
    }

    if access_token_type == AccessTokenType::App {
        if !option.tenant_access_token.is_empty() {
            return Err(LarkAPIError::Custom(
                "user token type not match tenant access token".to_string(),
            ));
        }
    }

    Ok(())
}
