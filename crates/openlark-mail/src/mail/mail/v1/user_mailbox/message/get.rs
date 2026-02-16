//! 获取邮件详情

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
pub struct GetMailboxMessageRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMailboxMessageResponse {
    pub data: Option<MessageData>,
}

impl ApiResponseTrait for GetMailboxMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub message_id: String,
    pub subject: String,
    pub body: String,
}

impl GetMailboxMessageRequest {
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetMailboxMessageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailboxMessageResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}",
            self.user_mailbox_id, self.message_id
        );
        let req: ApiRequest<GetMailboxMessageResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取邮件详情", "响应数据为空"))
    }
}
