//! 新增自定义角色
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateRoleRequest {
    pub role_name: String,
    pub table_perms: Vec<TablePerm>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TablePerm {
    pub table_id: String,
    pub perm: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateRoleResponse {
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Role {
    pub role_name: String,
    pub role_id: String,
    pub table_perms: Vec<TablePerm>,
}

impl ApiResponseTrait for CreateRoleResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct CreateRole {
    config: openlark_core::config::Config,
    app_token: String,
    req: CreateRoleRequest,
}

impl CreateRole {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            req: CreateRoleRequest::default(),
        }
    }

    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.req.role_name = role_name.into();
        self
    }

    pub fn table_perms(mut self, table_perms: Vec<TablePerm>) -> Self {
        self.req.table_perms = table_perms;
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<CreateRoleResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/roles",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
