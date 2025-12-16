use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response},
    error::DocsResult,
    req_option::RequestOption,
    constants::AccessTokenType,
};
use crate::service::DocsService;

// Builder
#[derive(Debug)]
pub struct UploadPartBuilder<'a> {
    client: &'a DocsService<'a>,
    upload_id: String,
    seq: i32,
    size: i32,
    data: Vec<u8>,
    checksum: Option<String>,
}

impl<'a> UploadPartBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, upload_id: impl Into<String>, seq: i32, size: i32, data: Vec<u8>) -> Self {
        Self { client, upload_id: upload_id.into(), seq, size, data, checksum: None }
    }

    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    pub async fn send(self) -> DocsResult<Response<UploadPartResponse>> {
        let mut req = ApiRequest::post("/open-apis/drive/v1/files/upload_part");
        // Simplified body
        let mut body = serde_json::Map::new();
        body.insert("upload_id".to_string(), serde_json::json!(self.upload_id));
        body.insert("seq".to_string(), serde_json::json!(self.seq));
        body.insert("size".to_string(), serde_json::json!(self.size));
        if let Some(c) = self.checksum {
             body.insert("checksum".to_string(), serde_json::json!(c));
        }
        req = req.body(serde_json::Value::Object(body));
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct UploadPartResponse {}

impl ApiResponseTrait for UploadPartResponse {}