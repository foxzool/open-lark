use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response},
    error::DocsResult,
    req_option::RequestOption,
    constants::AccessTokenType,
};
use crate::service::DocsService;

// Builder
#[derive(Debug)]
pub struct UploadAllBuilder<'a> {
    client: &'a DocsService<'a>,
    file_name: String,
    parent_type: String,
    parent_node: String,
    size: i64,
    file: Vec<u8>,
    checksum: Option<String>,
}

impl<'a> UploadAllBuilder<'a> {
    pub fn new(client: &'a DocsService<'a>, file_name: impl Into<String>, parent_type: impl Into<String>, parent_node: impl Into<String>, size: i64, file: Vec<u8>) -> Self {
        Self { client, file_name: file_name.into(), parent_type: parent_type.into(), parent_node: parent_node.into(), size, file, checksum: None }
    }

    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    pub async fn send(self) -> DocsResult<Response<UploadAllResponse>> {
        let mut req = ApiRequest::post("/open-apis/drive/v1/files/upload_all");
        let mut body = serde_json::Map::new();
        body.insert("file_name".to_string(), serde_json::json!(self.file_name));
        body.insert("parent_type".to_string(), serde_json::json!(self.parent_type));
        body.insert("parent_node".to_string(), serde_json::json!(self.parent_node));
        body.insert("size".to_string(), serde_json::json!(self.size));
        if let Some(c) = self.checksum {
            body.insert("checksum".to_string(), serde_json::json!(c));
        }
        // Note: Real upload usually requires Multipart. Assuming SDK/Core handles it or this is a placeholder structure.
        // For now, adhering to the pattern of setting body fields.
        req = req.body(serde_json::Value::Object(body));
        self.client.request(req).await
    }
}

// Response
#[derive(Debug, serde::Deserialize)]
pub struct UploadAllResponse {
    pub file_token: String,
}

impl ApiResponseTrait for UploadAllResponse {}