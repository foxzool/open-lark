//! 批量添加公共邮箱成员

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
pub struct BatchCreatePublicMailboxMemberRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePublicMailboxMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchCreatePublicMailboxMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchCreatePublicMailboxMemberRequest {
    pub fn new(config: Arc<Config>, public_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            public_mailbox_id: public_mailbox_id.into(),
            
        }
    }

    pub async fn execute(self) -> SDKResult<BatchCreatePublicMailboxMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreatePublicMailboxMemberResponse> {
        let path = format!("/open-apis/mail/v1/public_mailboxes/{{}}/members/batch_create", self.public_mailbox_id);
        let req: ApiRequest<BatchCreatePublicMailboxMemberResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量添加公共邮箱成员", "响应数据为空")
        })
    }
}
