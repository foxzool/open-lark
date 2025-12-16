//! 批量新增自定义角色的协作者
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/batch_create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchCreateRoleMemberRequest {
    pub members: Vec<AppRoleMemberId>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppRoleMemberId {
    pub type_: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchCreateRoleMemberResponse {}

impl ApiResponseTrait for BatchCreateRoleMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchCreateRoleMemberBuilder {
    api_req: ApiRequest<BatchCreateRoleMemberRequest>,
    app_token: String,
    role_id: String,
}

impl BatchCreateRoleMemberBuilder {
    pub fn new(app_token: impl ToString, role_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_role_member_batch_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.role_id = role_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_create",
            builder.app_token, builder.role_id
        );
        builder.api_req.body = Some(BatchCreateRoleMemberRequest::default());
        builder
    }

    pub fn members(mut self, members: Vec<AppRoleMemberId>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.members = members;
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
