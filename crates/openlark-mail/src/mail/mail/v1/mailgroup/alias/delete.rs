//! 删除邮件组别名

use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DeleteMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    alias_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupAliasResponse {
    pub data: Option<serde_json::Value>,
}

impl openlark_core::api::ApiResponseTrait for DeleteMailGroupAliasResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

impl DeleteMailGroupAliasRequest {
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        alias_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            alias_id: alias_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases/{}",
            self.mailgroup_id, self.alias_id
        );
        let req: ApiRequest<DeleteMailGroupAliasResponse> = ApiRequest::delete(&path);

        let _resp: Response<DeleteMailGroupAliasResponse> = Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteMailGroupAliasResponse { data: None })
    }
}
