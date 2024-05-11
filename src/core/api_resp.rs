use std::fmt::{Display, Formatter};

use bytes::Bytes;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::core::constants::{HTTP_HEADER_KEY_LOG_ID, HTTP_HEADER_KEY_REQUEST_ID};

#[derive(Default, Debug)]
pub struct ApiResp {
    pub status_code: u16,
    pub header: HeaderMap,
    pub raw_body: Bytes,
}

impl ApiResp {
    pub fn success(&self) -> bool {
        200 <= self.status_code && self.status_code < 300
    }
    pub fn request_id(&self) -> String {
        match self.header.get(HTTP_HEADER_KEY_LOG_ID) {
            None => self
                .header
                .get(HTTP_HEADER_KEY_REQUEST_ID)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            Some(log_id) => log_id.to_str().unwrap().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeError {
    pub code: i32,
    pub msg: String,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub err: Option<ErrorInfo>,
}

impl Display for CodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorInfo {
    #[serde(rename = "details", default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CodeErrorDetail>,
    #[serde(
        rename = "permission_violations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permission_violations: Vec<CodeErrorPermissionViolation>,
    #[serde(
        rename = "field_violations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_violations: Vec<CodeErrorFieldViolation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorDetail {
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorPermissionViolation {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorFieldViolation {
    #[serde(rename = "field", default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}
