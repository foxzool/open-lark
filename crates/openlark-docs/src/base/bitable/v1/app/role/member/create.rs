//! 新增自定义角色的协作者
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateRoleMemberRequest {
    pub member_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateRoleMemberResponse {}

impl ApiResponseTrait for CreateRoleMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct CreateRoleMember {
    config: openlark_core::config::Config,
    app_token: String,
    role_id: String,
    req: CreateRoleMemberRequest,
}

impl CreateRoleMember {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, role_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            role_id: role_id.into(),
            req: CreateRoleMemberRequest::default(),
        }
    }

    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.req.member_id = member_id.into();
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<CreateRoleMemberResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/roles/{}/members",
            self.config.base_url, self.app_token, self.role_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
