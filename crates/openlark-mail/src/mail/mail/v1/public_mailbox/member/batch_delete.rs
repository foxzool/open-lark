//! 批量删除公共邮箱成员

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BatchDeletePublicMailboxMemberRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeletePublicMailboxMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchDeletePublicMailboxMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchDeletePublicMailboxMemberRequest {
    pub fn new(config: Arc<Config>, public_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            public_mailbox_id: public_mailbox_id.into(),
            
        }
    }

    pub async fn execute(self) -> SDKResult<BatchDeletePublicMailboxMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchDeletePublicMailboxMemberResponse> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{{}}/members/batch_delete", self.public_mailbox_id);
        let req: ApiRequest<BatchDeletePublicMailboxMemberResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量删除公共邮箱成员", "响应数据为空")
        })
    }
}
