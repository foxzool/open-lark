use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::AppRoleService;
use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 删除自定义角色请求
#[derive(Debug, Serialize, Default)]
pub struct DeleteAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 自定义角色的id
    #[serde(skip)]
    role_id: String,
}

impl DeleteAppRoleRequest {
    pub fn builder() -> DeleteAppRoleRequestBuilder {
        DeleteAppRoleRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, role_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            role_id: role_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct DeleteAppRoleRequestBuilder {
    request: DeleteAppRoleRequest,
}

impl DeleteAppRoleRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 自定义角色的id
    pub fn role_id(mut self, role_id: impl ToString) -> Self {
        self.request.role_id = role_id.to_string();
        self
    }

    pub fn build(self) -> DeleteAppRoleRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    DeleteAppRoleRequestBuilder,
    AppRoleService,
    DeleteAppRoleRequest,
    BaseResponse<DeleteAppRoleResponse>,
    delete
);

/// 删除自定义角色响应
#[derive(Debug, Deserialize)]
pub struct DeleteAppRoleResponse {
    /// 删除的角色ID
    pub role_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除自定义角色
pub async fn delete_app_role(
    request: DeleteAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteAppRoleResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = format!(
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}",
        app_token = request.app_token,
        role_id = request.role_id
    );
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_app_role_request_builder() {
        let request = DeleteAppRoleRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
    }
}
