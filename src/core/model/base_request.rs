use std::collections::{HashMap, HashSet};

use reqwest::Method;
use serde::Serialize;
use serde_json::Value;

use crate::core::enum_type::AccessTokenType;

#[derive(Default, Debug, Clone)]
pub struct BaseRequest {
    pub(crate) http_method: Option<Method>,
    pub(crate) uri: Option<String>,
    pub(crate) token_types: HashSet<AccessTokenType>,
    pub(crate) paths: HashMap<String, String>,
    pub(crate) queries: Vec<(String, String)>,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Option<Value>,
    files: Option<HashMap<String, String>>,
}

impl BaseRequest {
    pub fn builder() -> BaseRequestBuilder {
        BaseRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct BaseRequestBuilder {
    base_request: BaseRequest,
}

impl BaseRequestBuilder {
    pub fn http_method(mut self, http_method: Method) -> Self {
        self.base_request.http_method = Some(http_method);
        self
    }

    pub fn uri(mut self, uri: impl ToString) -> Self {
        self.base_request.uri = Some(uri.to_string());
        self
    }

    pub fn token_types(mut self, token_types: HashSet<AccessTokenType>) -> Self {
        self.base_request.token_types = token_types;
        self
    }

    pub fn paths(mut self, paths: HashMap<String, String>) -> Self {
        self.base_request.paths = paths;
        self
    }

    pub fn queries(mut self, queries: Vec<(String, String)>) -> Self {
        self.base_request.queries = queries;
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.base_request.headers = headers;
        self
    }

    pub fn body(mut self, body: impl Serialize) -> Self {
        self.base_request.body = Some(serde_json::to_value(body).unwrap());
        self
    }

    pub fn files(mut self, files: HashMap<String, String>) -> Self {
        self.base_request.files = Some(files);
        self
    }

    pub fn build(self) -> BaseRequest {
        self.base_request
    }
}
