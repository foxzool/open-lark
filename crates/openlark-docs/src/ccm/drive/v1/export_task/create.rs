use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response},
    error::DocsResult,
    req_option::RequestOption,
    constants::AccessTokenType,
};
use crate::service::DocsService;

// Builder
#[derive(Debug)]
pub struct CreateBuilder<'a> {
    client: &'a DocsService<'a>,
    file_extension: String,
    token: String,
    type_: String,
    sub_id: Option<String>,
    reply_id: Option<String>,
}

impl<'a> CreateBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, file_extension: impl Into<String>, token: impl Into<String>, type_: impl Into<String>) -> Self {
        Self { client, file_extension: file_extension.into(), token: token.into(), type_: type_.into(), sub_id: None, reply_id: None }
    }

    // Setters

    pub async fn send(self) -> DocsResult<Response<CreateResponse>> {
        let mut req = ApiRequest::post("/open-apis/drive/v1/export_tasks");
        let mut body = serde_json::Map::new();
        body.insert("file_extension".to_string(), serde_json::json!(self.file_extension));
        body.insert("token".to_string(), serde_json::json!(self.token));
        body.insert("type".to_string(), serde_json::json!(self.type_));
        if let Some(s) = self.sub_id {
            body.insert("sub_id".to_string(), serde_json::json!(s));
        }
        req = req.body(serde_json::Value::Object(body));
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct CreateResponse {
    pub ticket: String,
}

impl ApiResponseTrait for CreateResponse {}