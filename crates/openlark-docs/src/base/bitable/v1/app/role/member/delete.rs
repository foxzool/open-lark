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

#[derive(Debug, Default)]
pub struct DeleteRoleMemberBuilder {
    api_req: ApiRequest<DeleteRoleMemberRequest>,
    app_token: String,
    role_id: String,
    member_id: String,
}

impl DeleteRoleMemberBuilder {
    pub fn new(app_token: impl ToString, role_id: impl ToString, member_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_role_member_delete".to_string();
        builder.api_req.method = "DELETE".to_string();
        builder.app_token = app_token.to_string();
        builder.role_id = role_id.to_string();
        builder.member_id = member_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/roles/{}/members/{}",
            builder.app_token, builder.role_id, builder.member_id
        );
        builder.api_req.body = None;
        builder
    }

    pub fn member_type(mut self, member_type: impl ToString) -> Self {
        if self.api_req.url.contains('?') {
            self.api_req.url.push_str(&format!("&member_type={}", member_type.to_string()));
        } else {
            self.api_req.url.push_str(&format!("?member_type={}", member_type.to_string()));
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
