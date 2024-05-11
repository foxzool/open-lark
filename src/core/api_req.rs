use std::collections::HashMap;
use bytes::Bytes;

use reqwest::Method;

use crate::core::constants::AccessTokenType;

#[derive(Debug, Clone, Default)]
pub struct ApiReq {
    pub http_method: Method,
    pub api_path: String,
    pub body: Bytes,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, Vec<String>>,
    pub supported_access_token_types: Vec<AccessTokenType>,
}
