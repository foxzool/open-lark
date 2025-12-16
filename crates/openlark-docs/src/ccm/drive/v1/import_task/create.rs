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
    file_token: String,
    type_: String,
    point: serde_json::Value,
}

impl<'a> CreateBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, file_extension: impl Into<String>, file_token: impl Into<String>, type_: impl Into<String>, point: serde_json::Value) -> Self {
        Self { client, file_extension: file_extension.into(), file_token: file_token.into(), type_: type_.into(), point }
    }

    pub async fn send(self) -> DocsResult<Response<CreateResponse>> {
        let mut req = ApiRequest::post("/open-apis/drive/v1/import_tasks");
        req = req.body(serde_json::json!({
            "file_extension": self.file_extension,
            "file_token": self.file_token,
            "type": self.type_,
            "point": self.point
        }));
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct CreateResponse {
    pub ticket: String,
}

impl ApiResponseTrait for CreateResponse {}