use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response},
    error::DocsResult,
    req_option::RequestOption,
    constants::AccessTokenType,
};
use crate::service::DocsService;

// Builder
#[derive(Debug)]
pub struct DownloadBuilder<'a> {
    client: &'a DocsService<'a>,
    file_token: String,
}

impl<'a> DownloadBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, file_token: impl Into<String>) -> Self {
        Self { client, file_token: file_token.into() }
    }

    pub async fn send(self) -> DocsResult<Response<DownloadResponse>> {
        let mut req = ApiRequest::get(format!("/open-apis/drive/v1/files/{}/download", self.file_token));
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct DownloadResponse {} // Stream response handled by core? Or returns bytes?
// The ApiResponseTrait might need adjustment for raw bytes, or we expect "Response" to wrap bytes.
// For now leaving empty struct as placeholder or implementing specific deserialization if needed.

impl ApiResponseTrait for DownloadResponse {}