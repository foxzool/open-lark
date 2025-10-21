use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::AppRoleService;

impl AppRoleService {
    /// 新增自定义角色
    pub async fn create(
        &self,
        request: CreateAppRoleRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateAppRoleResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = BITABLE_V1_ROLES.replace("{app_token}", &request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 新增自定义角色请求
#[derive(Debug, Serialize, Default)]
pub struct CreateAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 角色名称
    role_name: String,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 数据表权限
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TableRole {
    /// 数据表 id
    pub table_id: String,
    /// 权限
    pub role: String,
    /// 记录权限
    #[serde(skip_serializing_if = "Option::is_none")]
    rec_rule: Option<String>,
}

/// 数据表默认权限
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BlockRole {
    /// 多维表格数据表的唯一标识符
    pub block_id: String,
    /// 权限
    pub role: String,
}

impl CreateAppRoleRequest {
    pub fn builder() -> CreateAppRoleRequestBuilder {
        CreateAppRoleRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, role_name: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            role_name: role_name.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CreateAppRoleRequestBuilder {
    request: CreateAppRoleRequest,
}

impl CreateAppRoleRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 角色名称
    pub fn role_name(mut self, role_name: impl ToString) -> Self {
        self.request.role_name = role_name.to_string();
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

    pub fn build(mut self) -> CreateAppRoleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    CreateAppRoleRequestBuilder,
    AppRoleService,
    CreateAppRoleRequest,
    BaseResponse<CreateAppRoleResponse>,
    create
);

/// 自定义角色信息
#[derive(Debug, Deserialize)]
pub struct AppRole {
    /// 自定义角色的id
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 数据表权限
    pub table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    pub block_roles: Option<Vec<BlockRole>>,
}

/// 新增自定义角色响应
#[derive(Debug, Deserialize)]
pub struct CreateAppRoleResponse {
    /// 新增的自定义角色信息
    pub role: AppRole,
}

impl ApiResponseTrait for CreateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_app_role_request_builder() {
        let table_roles = vec![TableRole {
            table_id: "tblxxxxxx".to_string(),
            role: "editor".to_string(),
            rec_rule: Some("all".to_string()),
        }];

        let request = CreateAppRoleRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_name("测试自定义角色")
            .table_roles(table_roles)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_name, "测试自定义角色");
        assert!(request.table_roles.is_some());
    }
}
