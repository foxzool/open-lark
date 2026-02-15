//! 删除邮件组权限成员

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
pub struct DeleteMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    permission_member_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupPermissionMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl DeleteMailGroupPermissionMemberRequest {
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        permission_member_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            permission_member_id: permission_member_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/{}",
            self.mailgroup_id, self.permission_member_id
        );
        let req: ApiRequest<DeleteMailGroupPermissionMemberResponse> = ApiRequest::delete(&path);

        let _resp: Response<DeleteMailGroupPermissionMemberResponse> = Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteMailGroupPermissionMemberResponse { data: None })
    }
}

impl openlark_core::api::ApiResponseTrait for DeleteMailGroupPermissionMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}
