use std::collections::HashMap;

use bytes::Bytes;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::core::constants::{CONTENT_TYPE_JSON, CONTENT_TYPE_HEADER};
use crate::core::error::LarkAPIError;

pub trait BaseResponseTrait: Serialize + DeserializeOwned {
    fn code(&self) -> u16;
    fn msg(&self) -> &str;
}

#[derive(Default, Debug)]
pub struct BaseResponse<T: BaseResponseTrait> {
    pub code: u16,
    pub msg: String,
    pub headers: HashMap<String, String>,
    pub raw: Bytes,
    pub content: Option<T>,
}

impl<T: BaseResponseTrait> BaseResponse<T> {
    pub fn from_response(response: Response) -> Result<Self, LarkAPIError> {
        let status = response.status().as_u16();
        let mut code = if 200 <= status && status < 300 {
            0
        } else {
            status
        };
        let mut msg = "".to_string();

        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.as_str().to_lowercase(), v.to_str().unwrap().to_lowercase()))
            .collect();


        return if let Some(content_type) = headers.get(CONTENT_TYPE_HEADER.to_lowercase().as_str()) {
            if content_type.starts_with(CONTENT_TYPE_JSON) {
                let raw = response.bytes()?;
                let raw_clone = raw.clone().to_vec();
                let c: T = serde_json::from_slice(&raw_clone)?;
                Ok(Self {
                    code: c.code(),
                    msg: String::from(c.msg()),
                    headers,
                    raw,
                    content: Some(c),
                })
            } else {
                let raw = response.bytes()?;
                Ok(Self {
                    code,
                    msg,
                    headers,
                    raw,
                    content: None,
                })
            }
        } else {
            let raw = response.bytes()?;
            Ok(Self {
                code,
                msg,
                headers,
                raw,
                content: None,
            })
        }
    }

    pub fn success(&self) -> bool {
        self.code == 0
    }
}
