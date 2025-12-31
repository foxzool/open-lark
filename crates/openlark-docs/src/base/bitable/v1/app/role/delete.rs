/// Bitable 删除自定义角色
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/delete
/// doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 删除自定义角色请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeleteAppRoleRequest {
    config: Config,
    app_token: String,
    role_id: String,
}

impl DeleteAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    pub async fn execute(self) -> SDKResult<DeleteAppRoleResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.role_id.trim().is_empty() {
            return Err(validation_error("role_id", "role_id 不能为空"));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RoleDelete(self.app_token, self.role_id);

        let api_request: ApiRequest<DeleteAppRoleResponse> = ApiRequest::delete(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 删除自定义角色 Builder
pub struct DeleteAppRoleRequestBuilder {
    request: DeleteAppRoleRequest,
}

impl DeleteAppRoleRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteAppRoleRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn role_id(mut self, role_id: String) -> Self {
        self.request = self.request.role_id(role_id);
        self
    }

    pub fn build(self) -> DeleteAppRoleRequest {
        self.request
    }
}

/// 删除自定义角色响应（data 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteAppRoleResponse {}

impl ApiResponseTrait for DeleteAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
