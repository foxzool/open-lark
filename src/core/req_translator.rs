use std::collections::HashMap;

use async_recursion::async_recursion;
use reqwest::{multipart, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::core::{
    api_req::ApiRequest,
    config::Config,
    constants::{
        AccessTokenType, CONTENT_TYPE_HEADER, CUSTOM_REQUEST_ID, DEFAULT_CONTENT_TYPE,
        USER_AGENT_HEADER,
    },
    error::LarkAPIError,
    // multi_part::MultipartBuilder,
    req_option::RequestOption,
    token_manager::TOKEN_MANAGER,
    utils::user_agent,
};

pub struct ReqTranslator;

impl ReqTranslator {
    #[async_recursion]
    pub async fn translate(
        req: &mut ApiRequest,
        access_token_type: AccessTokenType,
        config: &Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let client = reqwest::Client::new();

        let path = format!("{}{}", config.base_url, req.api_path);
        let query_params = req
            .query_params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<_>>();
        let url = Url::parse_with_params(&path, query_params)?;

        let mut req_builder = client.request(req.http_method.clone(), url.as_ref());
        // .send_bytes(&req.body);
        if !option.request_id.is_empty() {
            req_builder = req_builder.header(CUSTOM_REQUEST_ID, &option.request_id.clone());
        }
        for (k, v) in &option.header {
            req_builder = req_builder.header(k, v)
        }

        for (k, v) in &config.header {
            req_builder = req_builder.header(k, v)
        }
        req_builder = req_builder.header(USER_AGENT_HEADER, &user_agent());

        match access_token_type {
            AccessTokenType::None => {}
            AccessTokenType::App => {
                let mut app_access_token = option.app_access_token.clone();
                if config.enable_token_cache && app_access_token.is_empty() {
                    {
                        let mut token_manager = TOKEN_MANAGER.lock().await;
                        app_access_token = token_manager
                            .get_app_access_token(config, &option.app_ticket)
                            .await?
                    }
                }
                req_builder = authorization_to_header(req_builder, &app_access_token);
            }
            AccessTokenType::Tenant => {
                let mut tenant_access_token = option.tenant_access_token.clone();
                if config.enable_token_cache {
                    {
                        let mut token_manager = TOKEN_MANAGER.lock().await;
                        tenant_access_token = token_manager
                            .get_tenant_access_token(config, &option.tenant_key, &option.app_ticket)
                            .await?;
                    }
                }

                req_builder = authorization_to_header(req_builder, &tenant_access_token);
            }
            AccessTokenType::User => {
                req_builder = authorization_to_header(req_builder, &option.user_access_token);
            }
        }

        if !req.file.is_empty() {
            let json_value = serde_json::from_slice::<Value>(&req.body.clone())?;

            if let Some(form_obj) = json_value.as_object() {
                let mut form = multipart::Form::new();
                let file_name = form_obj
                    .get("file_name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                let file_part = multipart::Part::bytes(req.file.clone()).file_name(file_name);
                form = form.part("file", file_part);

                for (k, v) in form_obj.iter() {
                    if v == &Value::Null {
                        continue;
                    }
                    match v {
                        Value::String(s) => {
                            form = form.text(k.to_string(), s.to_string());
                        }
                        Value::Number(n) => {
                            form = form.text(k.to_string(), n.to_string().as_str().to_string());
                        }
                        Value::Bool(b) => {
                            form = form.text(k.to_string(), b.to_string().as_str().to_string());
                        }
                        _ => {}
                    }
                }

                form = form.percent_encode_noop();
                req_builder = req_builder.multipart(form);
            }
        } else {
            req_builder = req_builder.header(CONTENT_TYPE_HEADER, DEFAULT_CONTENT_TYPE);
        }

        Ok(req_builder)
    }
}

fn authorization_to_header(req: RequestBuilder, token: &str) -> RequestBuilder {
    req.header("Authorization", &format!("Bearer {token}"))
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

// async fn to_form_data(body: Bytes) -> Result<Form, LarkAPIError> {
//     let data = serde_json::from_slice::<Value>(&body)?;
//
//     let mut form = Form::new();
//
//     if let Value::Object(map) = data {
//         for (key, val) in map {
//             if key == "file" {
//                 let file_name = val.as_str().unwrap().to_string().clone();
//                 let file = File::open(file_name.clone()).await?;
//                 let stream = FramedRead::new(file, BytesCodec::new());
//                 let file_body = Body::wrap_stream(stream);
//
//                 //make form part of file
//                 let some_file = multipart::Part::stream(file_body)
//                     .file_name(file_name)
//                     .mime_str("application/octet-stream")?;
//                 form = form.part("file", some_file);
//                 continue;
//             }
//             match val {
//                 Value::String(s) => {
//                     form = form.text(key, s.clone());
//                 }
//                 Value::Number(n) => {
//                     form = form.text(key, n.to_string());
//                 }
//                 Value::Bool(b) => {
//                     form = form.text(key, b.to_string());
//                 }
//                 _ => {}
//             }
//         }
//     }
//
//     Ok(form)
// }
