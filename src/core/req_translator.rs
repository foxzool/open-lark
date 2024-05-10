use std::collections::HashMap;

use bytes::Bytes;
use reqwest::blocking::{Request, RequestBuilder};
use reqwest::blocking::multipart::Form;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::core::api_req::ApiReq;
use crate::core::config::Config;
use crate::core::constants::{AccessTokenType, CONTENT_TYPE_HEADER, CUSTOM_REQUEST_ID, DEFAULT_CONTENT_TYPE, USER_AGENT_HEADER};
use crate::core::error::LarkAPIError;
use crate::core::req_option::RequestOption;
use crate::core::token_manager::TOKEN_MANAGER;
use crate::core::utils::user_agent;

pub struct ReqTranslator;

impl ReqTranslator {
    pub fn translate(
        req: &ApiReq,
        access_token_type: AccessTokenType,
        config: &Config,
        option: &RequestOption,
    ) -> Result<Request, LarkAPIError> {
        let client = reqwest::blocking::Client::new();

        let path = format!("{}/{}", config.base_url, req.api_path);
        let query_params = req
            .query_params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<_>>();
        let url = Url::parse_with_params(&path, query_params)?;

        let mut req_builder = client.request(req.http_method.clone(), url).body(req.body.clone());
        if !option.request_id.is_empty() {
            req_builder = req_builder.header(CUSTOM_REQUEST_ID, option.request_id.clone());
        }
        for (k, v) in &option.header {
            req_builder = req_builder.header(k, v)
        }

        for (k, v) in &config.header {
            req_builder = req_builder.header(k, v)
        }
        req_builder = req_builder.header(USER_AGENT_HEADER, user_agent());


        match access_token_type {
            AccessTokenType::None => {}
            AccessTokenType::App => {
                let mut app_access_token = option.app_access_token.clone();
                if config.enable_token_cache && app_access_token.is_empty() {
                    app_access_token = TOKEN_MANAGER
                        .lock()
                        .unwrap()
                        .get_app_access_token(config, &option.app_ticket)?
                }
                req_builder = authorization_to_header(req_builder, &app_access_token);
            }
            AccessTokenType::Tenant => {
                let mut tenant_access_token = option.tenant_access_token.clone();
                if config.enable_token_cache {
                    tenant_access_token = TOKEN_MANAGER.lock().unwrap().get_tenant_access_token(
                        config,
                        &option.tenant_key,
                        &option.app_ticket,
                    )?;
                }
                req_builder = authorization_to_header(req_builder, &tenant_access_token);
            }
            AccessTokenType::User => {
                req_builder = authorization_to_header(req_builder, &option.user_access_token);
            }
        }

        let mut file_upload = false;
        let body = req.body.clone();
        if let Ok(_) = serde_json::from_slice::<FormData>(&body) {
            file_upload = true;
        } else {
            if option.file_upload {
                file_upload = true;
            }
        }

        if file_upload {
            req_builder = req_builder.multipart(to_form_data(body)?);
        } else {
            req_builder = req_builder.header(CONTENT_TYPE_HEADER, DEFAULT_CONTENT_TYPE);
            req_builder = req_builder.body(req.body.clone())
        }

        Ok(req_builder.build()?)
    }
}

fn authorization_to_header(req: RequestBuilder, token: &str) -> RequestBuilder {
    req.header("Authorization", format!("Bearer {token}"))
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct FormData {
    fields: HashMap<String, Value>,
    data: Option<Data>,
}

impl FormData {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Data {
    content: Vec<u8>,
    content_type: String,
}

fn to_form_data(body: Bytes) -> Result<Form, LarkAPIError> {
    let mut data = serde_json::from_slice::<Value>(&body)?;

    let mut form = Form::new();

    if let Value::Object(map) = data {
        for (key, val) in map {
            if key == "file" {
                form = form.file("file", val.as_str().unwrap())?;
                continue;
            }
            match val {
                Value::String(s) => {
                    form = form.text(key, s.clone());
                }
                Value::Number(n) => {
                    form = form.text(key, n.to_string());
                }
                Value::Bool(b) => {
                    form = form.text(key, b.to_string());
                }
                _ => {}
            }
        }
    }

    Ok(form)
}
