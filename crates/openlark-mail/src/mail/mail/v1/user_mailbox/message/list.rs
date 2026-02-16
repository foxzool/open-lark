//! 列出邮件

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
pub struct ListMailboxMessageRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailboxMessageResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListMailboxMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListMailboxMessageRequest {
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<ListMailboxMessageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailboxMessageResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages",
            self.user_mailbox_id
        );
        let req: ApiRequest<ListMailboxMessageResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("列出邮件", "响应数据为空"))
    }
}
