//! 删除公共邮箱所有成员

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ClearPublicMailboxMemberRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearPublicMailboxMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl ClearPublicMailboxMemberRequest {
    pub fn new(config: Arc<Config>, public_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            public_mailbox_id: public_mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<ClearPublicMailboxMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ClearPublicMailboxMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members/clear",
            self.public_mailbox_id
        );
        let req: ApiRequest<ClearPublicMailboxMemberResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp)
    }
}
