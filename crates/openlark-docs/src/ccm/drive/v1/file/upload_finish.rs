use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response},
    error::DocsResult,
    req_option::RequestOption,
    constants::AccessTokenType,
};
use crate::service::DocsService;

// Builder
#[derive(Debug)]
pub struct UploadFinishBuilder<'a> {
    client: &'a DocsService<'a>,
    upload_id: String,
    block_num: i32,
}

impl<'a> UploadFinishBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, upload_id: impl Into<String>, block_num: i32) -> Self {
        Self { client, upload_id: upload_id.into(), block_num }
    }

    pub async fn send(self) -> DocsResult<Response<UploadFinishResponse>> {
        let mut req = ApiRequest::post("/open-apis/drive/v1/files/upload_finish");
        req = req.body(serde_json::json!({
            "upload_id": self.upload_id,
            "block_num": self.block_num
        }));
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct UploadFinishResponse {
    pub file_token: String,
}

impl ApiResponseTrait for UploadFinishResponse {}