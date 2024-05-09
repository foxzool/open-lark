use std::collections::HashMap;
use log::debug;

use reqwest::header::HeaderMap;

use crate::core::constants::{AUTHORIZATION, PROJECT, USER_AGENT, VERSION};
use crate::core::enum_type::AccessTokenType;
use crate::core::model::{BaseRequest, Config, RawResponse, RequestOption};

pub struct Transport;

impl Transport {
    pub fn execute(
        config: &Config,
        req: &BaseRequest,
        option: Option<RequestOption>,
    ) -> RawResponse {
        let option = option.unwrap_or_default();

        // 拼接url
        let url = build_url(
            &config.domain,
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
            .send()
            .unwrap();



        let queries_json = serde_json::to_string(&req.queries).unwrap();

        debug!(
            "{} {} {}, headers: {:?}, queries: {:?}, body: {:?} ",
            req.http_method.clone().unwrap_or_default(),
            url,
            response.status(),
            headers,
            queries_json,
            data
        );



        let mut resp = RawResponse::default();
        resp.status_code = response.status().as_u16();

        resp.headers = response.headers().iter().map(|(k,v)| {
            (k.as_str().to_string(), v.to_str().unwrap().to_string())
        }).collect::<HashMap<String, String>>();
        resp.content = Some(response.bytes().unwrap());
        resp
    }
}

fn build_url(domain: &str, uri: &str, paths: &HashMap<String, String>) -> String {
    let mut uri = uri.to_string();
    for (key, value) in paths {
        uri = uri.replace(&(":".to_string() + key), value);
    }

    domain.to_string() + &uri
}

fn build_header(request: &BaseRequest, option: &RequestOption) -> HeaderMap {
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
            AccessTokenType::TENANT => {
                headers.insert(
                    AUTHORIZATION,
                    format!("Bearer {}", option.tenant_access_token.as_ref().unwrap())
                        .parse()
                        .unwrap(),
                );
            }
            AccessTokenType::APP => {
                headers.insert(
                    AUTHORIZATION,
                    format!("Bearer {}", option.app_access_token.as_ref().unwrap())
                        .parse()
                        .unwrap(),
                );
            }
            AccessTokenType::USER => {
                headers.insert(
                    AUTHORIZATION,
                    format!("Bearer {}", option.user_access_token.as_ref().unwrap())
                        .parse()
                        .unwrap(),
                );
            }
        }
    }

    headers
}
