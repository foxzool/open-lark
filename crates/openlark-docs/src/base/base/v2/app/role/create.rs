//! 新增自定义角色
//!
//! doc: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/create-2

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateRoleRequest {
    pub role_name: String,
    pub table_perms: Vec<TablePerm>,
    pub block_perms: Option<Vec<BlockPerm>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TablePerm {
    pub table_id: String,
    pub perm: String, // e.g. "view", "edit", "none"
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BlockPerm {
    pub block_id: String,
    pub perm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateRoleResponse {
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Role {
    pub role_id: String,
    pub role_name: String,
    pub table_perms: Vec<TablePerm>,
    pub block_perms: Option<Vec<BlockPerm>>,
}

impl ApiResponseTrait for CreateRoleResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct CreateRoleBuilder {
    api_req: ApiRequest<CreateRoleRequest>,
    app_token: String,
}

impl CreateRoleBuilder {
    pub fn new(app_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "base_role_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/base/v2/apps/{}/roles",
            builder.app_token
        );
        builder.api_req.body = Some(CreateRoleRequest::default());
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

    pub fn block_perms(mut self, block_perms: Vec<BlockPerm>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.block_perms = Some(block_perms);
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
