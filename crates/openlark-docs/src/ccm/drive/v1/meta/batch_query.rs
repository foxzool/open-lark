use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response},
    error::DocsResult,
    req_option::RequestOption,
    constants::AccessTokenType,
};
use crate::service::DocsService;

// Builder
#[derive(Debug)]
pub struct BatchQueryBuilder<'a> {
    client: &'a DocsService<'a>,
    request_docs: Vec<serde_json::Value>,
    with_url: Option<bool>,
}

impl<'a> BatchQueryBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, request_docs: Vec<serde_json::Value>) -> Self {
        Self { client, request_docs, with_url: None }
    }

    pub fn with_url(mut self, with_url: bool) -> Self {
        self.with_url = Some(with_url);
        self
    }

    pub async fn send(self) -> DocsResult<Response<BatchQueryResponse>> {
        let mut req = ApiRequest::post("/open-apis/drive/v1/metas/batch_query");
        let mut body = serde_json::Map::new();
        body.insert("request_docs".to_string(), serde_json::json!(self.request_docs));
        if let Some(w) = self.with_url {
            body.insert("with_url".to_string(), serde_json::json!(w));
        }
        req = req.body(serde_json::Value::Object(body));
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct BatchQueryResponse {
    pub metas: Vec<serde_json::Value>,
    pub failed_list: Vec<serde_json::Value>,
}

impl ApiResponseTrait for BatchQueryResponse {}