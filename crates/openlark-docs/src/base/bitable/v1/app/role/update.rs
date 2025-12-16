//! 更新自定义角色
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role/update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateRoleRequest {
    pub role_name: String,
    pub table_perms: Vec<TablePerm>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TablePerm {
    pub table_id: String,
    pub perm: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateRoleResponse {
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Role {
    pub role_name: String,
    pub role_id: String,
    pub table_perms: Vec<TablePerm>,
}

impl ApiResponseTrait for UpdateRoleResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UpdateRoleBuilder {
    api_req: ApiRequest<UpdateRoleRequest>,
    app_token: String,
    role_id: String,
}

impl UpdateRoleBuilder {
    pub fn new(app_token: impl ToString, role_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_role_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.app_token = app_token.to_string();
        builder.role_id = role_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/roles/{}",
            builder.app_token, builder.role_id
        );
        builder.api_req.body = Some(UpdateRoleRequest::default());
        builder
    }

    pub fn role_name(mut self, role_name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.role_name = role_name.to_string();
        }
        self
    }

    pub fn table_perms(mut self, table_perms: Vec<TablePerm>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.table_perms = table_perms;
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
