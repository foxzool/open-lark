//! 删除自定义角色的协作者
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteRoleMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteRoleMemberResponse {}

impl ApiResponseTrait for DeleteRoleMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct DeleteRoleMember {
    config: openlark_core::config::Config,
    app_token: String,
    role_id: String,
    member_id: String,
    req: DeleteRoleMemberRequest,
}

impl DeleteRoleMember {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, role_id: impl Into<String>, member_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            role_id: role_id.into(),
            member_id: member_id.into(),
            req: DeleteRoleMemberRequest::default(),
        }
    }

    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.req.member_type = Some(member_type.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<DeleteRoleMemberResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/roles/{}/members/{}",
            self.config.base_url, self.app_token, self.role_id, self.member_id
        );
        let request = ApiRequest::delete(&url).query(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
