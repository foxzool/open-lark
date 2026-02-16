//! 对收信规则进行排序

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
pub struct ReorderMailboxRuleRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderMailboxRuleResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ReorderMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ReorderMailboxRuleRequest {
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<ReorderMailboxRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ReorderMailboxRuleResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/rules/reorder",
            self.user_mailbox_id
        );
        let req: ApiRequest<ReorderMailboxRuleResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("对收信规则进行排序", "响应数据为空")
        })
    }
}
