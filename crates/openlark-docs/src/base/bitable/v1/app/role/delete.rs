//! 删除自定义角色
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role/delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteRoleRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteRoleResponse {}

impl ApiResponseTrait for DeleteRoleResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct DeleteRoleBuilder {
    api_req: ApiRequest<DeleteRoleRequest>,
    app_token: String,
    role_id: String,
}

impl DeleteRoleBuilder {
    pub fn new(app_token: impl ToString, role_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_role_delete".to_string();
        builder.api_req.method = "DELETE".to_string();
        builder.app_token = app_token.to_string();
        builder.role_id = role_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/roles/{}",
            builder.app_token, builder.role_id
        );
        builder.api_req.body = Some(DeleteRoleRequest::default());
        builder
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
