//! 删除用户邮箱别名

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
pub struct DeleteMailboxAliasRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    alias_id: String,
    delete_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailboxAliasResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteMailboxAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteMailboxAliasRequest {
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        alias_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            alias_id: alias_id.into(),
            delete_id: String::new(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteMailboxAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailboxAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/aliases/{}",
            self.user_mailbox_id, self.alias_id
        );
        let req: ApiRequest<DeleteMailboxAliasResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("删除用户邮箱别名", "响应数据为空")
        })
    }
}
