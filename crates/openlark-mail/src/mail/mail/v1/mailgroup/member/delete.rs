//! 删除邮件组成员

use openlark_core::{
    api::Response,
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DeleteMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    member_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl DeleteMailGroupMemberRequest {
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            member_id: member_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/{}",
            self.mailgroup_id, self.member_id
        );
        let req: ApiRequest<DeleteMailGroupMemberResponse> = ApiRequest::delete(&path);

        let _resp: Response<DeleteMailGroupMemberResponse> = Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteMailGroupMemberResponse { data: None })
    }
}

impl openlark_core::api::ApiResponseTrait for DeleteMailGroupMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}
