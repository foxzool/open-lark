use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::AppRoleService;
use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::Endpoints,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::bitable::v1::app_role::{AppRole, BlockRole, TableRole},
};

/// 更新自定义角色请求
#[derive(Debug, Serialize, Default)]
pub struct UpdateAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 自定义角色的id
    #[serde(skip)]
    role_id: String,
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<String>,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

impl UpdateAppRoleRequest {
    pub fn builder() -> UpdateAppRoleRequestBuilder {
        UpdateAppRoleRequestBuilder::default()
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
pub struct UpdateAppRoleRequestBuilder {
    request: UpdateAppRoleRequest,
}

impl UpdateAppRoleRequestBuilder {
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

    /// 角色名称
    pub fn role_name(mut self, role_name: impl ToString) -> Self {
        self.request.role_name = Some(role_name.to_string());
        self
    }

    /// 数据表权限
    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.request.table_roles = Some(table_roles);
        self
    }

    /// 数据表默认权限
    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.request.block_roles = Some(block_roles);
        self
    }

    pub fn build(mut self) -> UpdateAppRoleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    UpdateAppRoleRequestBuilder,
    AppRoleService,
    UpdateAppRoleRequest,
    BaseResponse<UpdateAppRoleResponse>,
    update
);

/// 更新自定义角色响应
#[derive(Debug, Deserialize)]
pub struct UpdateAppRoleResponse {
    /// 更新后的自定义角色信息
    pub role: AppRole,
}

impl ApiResponseTrait for UpdateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新自定义角色
pub async fn update_app_role(
    request: UpdateAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdateAppRoleResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PUT;
    api_req.api_path = Endpoints::BITABLE_V1_ROLE_UPDATE
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_app_role_request_builder() {
        let request = UpdateAppRoleRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .role_name("更新后的角色名称")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.role_name, Some("更新后的角色名称".to_string()));
    }
}
