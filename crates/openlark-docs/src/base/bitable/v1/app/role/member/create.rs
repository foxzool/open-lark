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

#[derive(Debug, Default)]
pub struct CreateRoleMemberBuilder {
    api_req: ApiRequest<CreateRoleMemberRequest>,
    app_token: String,
    role_id: String,
}

impl CreateRoleMemberBuilder {
    pub fn new(app_token: impl ToString, role_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_role_member_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.role_id = role_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/roles/{}/members",
            builder.app_token, builder.role_id
        );
        builder.api_req.body = Some(CreateRoleMemberRequest::default());
        builder
    }

    pub fn member_id(mut self, member_id: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.member_id = member_id.to_string();
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
