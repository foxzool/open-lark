//! 删除收信规则

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
pub struct DeleteMailboxRuleRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    rule_id: String,
    delete_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailboxRuleResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteMailboxRuleRequest {
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>, rule_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            rule_id: rule_id.into(),
            delete_id: String::new(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteMailboxRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailboxRuleResponse> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{}/rules/{}", self.user_mailbox_id, self.rule_id);
        let req: ApiRequest<DeleteMailboxRuleResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("删除收信规则", "响应数据为空")
        })
    }
}
