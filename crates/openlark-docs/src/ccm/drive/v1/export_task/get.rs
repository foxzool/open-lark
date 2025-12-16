use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response},
    error::DocsResult,
    req_option::RequestOption,
    constants::AccessTokenType,
};
use crate::service::DocsService;

// Builder
#[derive(Debug)]
pub struct GetBuilder<'a> {
    client: &'a DocsService<'a>,
    ticket: String,
    token: String,
}

impl<'a> GetBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, ticket: impl Into<String>, token: impl Into<String>) -> Self {
        Self { client, ticket: ticket.into(), token: token.into() }
    }

    pub async fn send(self) -> DocsResult<Response<GetResponse>> {
        let mut req = ApiRequest::get(format!("/open-apis/drive/v1/export_tasks/{}", self.ticket));
        req = req.query("token", &self.token);
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct GetResponse {
    pub result: serde_json::Value,
}

impl ApiResponseTrait for GetResponse {}