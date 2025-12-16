use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response},
    error::DocsResult,
    req_option::RequestOption,
    constants::AccessTokenType,
};
use crate::service::DocsService;

// Builder
#[derive(Debug)]
pub struct UploadPrepareBuilder<'a> {
    client: &'a DocsService<'a>,
    file_name: String,
    parent_type: String,
    parent_node: String,
    size: i64,
}

impl<'a> UploadPrepareBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, file_name: impl Into<String>, parent_type: impl Into<String>, parent_node: impl Into<String>, size: i64) -> Self {
        Self { client, file_name: file_name.into(), parent_type: parent_type.into(), parent_node: parent_node.into(), size }
    }

    pub async fn send(self) -> DocsResult<Response<UploadPrepareResponse>> {
        let mut req = ApiRequest::post("/open-apis/drive/v1/files/upload_prepare");
        req = req.body(serde_json::json!({
            "file_name": self.file_name,
            "parent_type": self.parent_type,
            "parent_node": self.parent_node,
            "size": self.size
        }));
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct UploadPrepareResponse {
    pub upload_id: String,
    pub block_size: i32,
    pub block_num: i32,
}

impl ApiResponseTrait for UploadPrepareResponse {}