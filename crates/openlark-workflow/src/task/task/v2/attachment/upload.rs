//! 上传附件
//!
//! docPath: https://open.feishu.cn/document/task-v2/attachment/upload

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UploadAttachmentBody { pub resource_id: Option<String>, pub file_token: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UploadAttachmentResponse { pub attachment: Option<serde_json::Value> }
impl ApiResponseTrait for UploadAttachmentResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct UploadAttachmentRequest { config: Arc<Config>, body: UploadAttachmentBody }
impl UploadAttachmentRequest { pub fn new(config: Arc<Config>) -> Self { Self { config, body: UploadAttachmentBody::default() } } pub fn body(mut self, body: UploadAttachmentBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<UploadAttachmentResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<UploadAttachmentResponse> { let request = ApiRequest::<UploadAttachmentResponse>::post("/open-apis/task/v2/attachments/upload").body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("上传附件", "响应数据为空")) } }
