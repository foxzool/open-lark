use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ureq::Request;
use url::Url;

use crate::core::{
    api_req::ApiReq,
    config::Config,
    constants::{
        AccessTokenType, CONTENT_TYPE_HEADER, CUSTOM_REQUEST_ID, DEFAULT_CONTENT_TYPE,
        USER_AGENT_HEADER,
    },
    error::LarkAPIError,
    multi_part::MultipartBuilder,
    req_option::RequestOption,
    token_manager::TOKEN_MANAGER,
    utils::user_agent,
};

pub struct ReqTranslator;

impl ReqTranslator {
    pub fn translate(
        req: &mut ApiReq,
        access_token_type: AccessTokenType,
        config: &Config,
        option: &RequestOption,
    ) -> Result<Request, LarkAPIError> {
        let client = ureq::Agent::new();

        let path = format!("{}{}", config.base_url, req.api_path);
        let query_params = req
            .query_params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<_>>();
        let url = Url::parse_with_params(&path, query_params)?;

        let mut req_builder = client.request(&req.http_method, url.as_ref());
        // .send_bytes(&req.body);
        if !option.request_id.is_empty() {
            req_builder = req_builder.set(CUSTOM_REQUEST_ID, &option.request_id.clone());
        }
        for (k, v) in &option.header {
            req_builder = req_builder.set(k, v)
        }

        for (k, v) in &config.header {
            req_builder = req_builder.set(k, v)
        }
        req_builder = req_builder.set(USER_AGENT_HEADER, &user_agent());

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

        if !req.file.is_empty() {
            let json_value = serde_json::from_slice::<Value>(&req.body)?;

            if let Some(form_obj) = json_value.as_object() {
                let mut builder = MultipartBuilder::new();
                let file_name = form_obj["file_name"].as_str().unwrap();
                // builder = builder.add_file("file", "target/1.txt").unwrap();
                builder = builder.load_file(&file_name, req.file.clone())?;

                for (k, v) in form_obj.iter() {
                    if v == &Value::Null {
                        continue;
                    }
                    match v {
                        Value::String(s) => {
                            builder = builder.add_text(k, s)?;
                        }
                        Value::Number(n) => {
                            builder = builder.add_text(k, n.to_string().as_str())?;
                        }
                        Value::Bool(b) => {
                            builder = builder.add_text(k, b.to_string().as_str())?;
                        }
                        _ => {}
                    }
                }

                let (content_type, data) = builder.finish().unwrap();
                req.body = data;

                req_builder = req_builder.set(CONTENT_TYPE_HEADER, &content_type);
            }
        } else {
            req_builder = req_builder.set(CONTENT_TYPE_HEADER, DEFAULT_CONTENT_TYPE);
            // req_builder = req_builder.body(req.body.clone())
        }

        Ok(req_builder)
    }
}

fn authorization_to_header(req: Request, token: &str) -> Request {
    req.set("Authorization", &format!("Bearer {token}"))
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
