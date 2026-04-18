//! 删除附件
//!
//! docPath: https://open.feishu.cn/document/task-v2/attachment/delete

use crate::common::api_utils::*;
use crate::v2::attachment::models::DeleteAttachmentResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DeleteAttachmentRequest {
    config: Arc<Config>,
    attachment_guid: String,
}

impl DeleteAttachmentRequest {
    pub fn new(config: Arc<Config>, attachment_guid: impl Into<String>) -> Self {
        Self {
            config,
            attachment_guid: attachment_guid.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteAttachmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteAttachmentResponse> {
        validate_required!(self.attachment_guid.trim(), "附件 GUID 不能为空");

        let path = format!("/open-apis/task/v2/attachments/{}", self.attachment_guid);
        let request = ApiRequest::<DeleteAttachmentResponse>::delete(&path);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除附件")
    }
}

impl ApiResponseTrait for DeleteAttachmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
