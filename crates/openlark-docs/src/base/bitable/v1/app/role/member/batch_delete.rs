//! 批量删除自定义角色的协作者
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/batch_delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteRoleMemberRequest {
    pub members: Vec<AppRoleMemberId>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppRoleMemberId {
    pub type_: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteRoleMemberResponse {}

impl ApiResponseTrait for BatchDeleteRoleMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct BatchDeleteRoleMember {
    config: openlark_core::config::Config,
    app_token: String,
    role_id: String,
    req: BatchDeleteRoleMemberRequest,
}

impl BatchDeleteRoleMember {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, role_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            role_id: role_id.into(),
            req: BatchDeleteRoleMemberRequest::default(),
        }
    }

    pub fn members(mut self, members: Vec<AppRoleMemberId>) -> Self {
        self.req.members = members;
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<BatchDeleteRoleMemberResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_delete",
            self.config.base_url, self.app_token, self.role_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
