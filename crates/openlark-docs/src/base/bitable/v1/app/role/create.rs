//! Bitable 新增自定义角色
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::{BlockRole, Role, TableRole};

/// 新增自定义角色请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CreateAppRoleRequest {
    config: Config,
    app_token: String,
    role_name: String,
    table_roles: Vec<TableRole>,
    block_roles: Option<Vec<BlockRole>>,
}

impl CreateAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_name: String::new(),
            table_roles: Vec::new(),
            block_roles: None,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn role_name(mut self, role_name: String) -> Self {
        self.role_name = role_name;
        self
    }

    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.table_roles = table_roles;
        self
    }

    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.block_roles = Some(block_roles);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateAppRoleResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.role_name.trim().is_empty() {
            return Err(validation_error("role_name", "role_name 不能为空"));
        }
        if self.table_roles.is_empty() {
            return Err(validation_error("table_roles", "table_roles 不能为空"));
        }
        if self.table_roles.len() > 100 {
            return Err(validation_error("table_roles", "table_roles 最多 100 项"));
        }
        if let Some(ref block_roles) = self.block_roles {
            if block_roles.len() > 100 {
                return Err(validation_error("block_roles", "block_roles 最多 100 项"));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RoleCreate(self.app_token.clone());

        let api_request: ApiRequest<CreateAppRoleResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&CreateAppRoleRequestBody {
            role_name: self.role_name,
            table_roles: self.table_roles,
            block_roles: self.block_roles,
        })?);

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 新增自定义角色 Builder
pub struct CreateAppRoleRequestBuilder {
    request: CreateAppRoleRequest,
}

impl CreateAppRoleRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateAppRoleRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn role_name(mut self, role_name: String) -> Self {
        self.request = self.request.role_name(role_name);
        self
    }

    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.request = self.request.table_roles(table_roles);
        self
    }

    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.request = self.request.block_roles(block_roles);
        self
    }

    pub fn build(self) -> CreateAppRoleRequest {
        self.request
    }
}

#[derive(Serialize)]
pub struct CreateAppRoleRequestBody {
    role_name: String,
    table_roles: Vec<TableRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 新增自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppRoleResponse {
    pub role: Role,
}

impl ApiResponseTrait for CreateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
