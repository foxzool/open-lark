use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::hire::*,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::hire::models::{PageResponse, Role, RoleListRequest, UserRole};

/// 权限服务
pub struct AuthService {
    pub config: Config,
}

/// 角色详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDetailResponse {
    /// 角色信息
    pub role: Role,
}

impl ApiResponseTrait for RoleDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 角色列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListResponse {
    /// 角色列表
    #[serde(flatten)]
    pub roles: PageResponse<Role>,
}

impl ApiResponseTrait for RoleListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户角色列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleListResponse {
    /// 用户角色列表
    #[serde(flatten)]
    pub user_roles: PageResponse<UserRole>,
}

impl ApiResponseTrait for UserRoleListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AuthService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取角色详情
    ///
    /// 该接口用于获取指定角色的详细信息，包括角色名称、
    /// 描述、权限列表等完整数据。
    ///
    /// # 参数
    ///
    /// - `role_id`: 角色ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let role_id = "role_123456";
    /// let response = client.hire.recruitment_config.auth.get_role_detail(role_id, None).await?;
    /// ```
    pub async fn get_role_detail(
        &self,
        role_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<RoleDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_ROLE_GET, "role_id", role_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取角色列表
    ///
    /// 该接口用于获取系统中的角色列表，包括角色基本信息
    /// 和权限配置，用于权限管理和用户角色分配。
    ///
    /// # 参数
    ///
    /// - `request`: 角色列表查询请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::RoleListRequest;
    ///
    /// let request = RoleListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    /// };
    ///
    /// let response = client.hire.recruitment_config.auth.list_roles(request, None).await?;
    /// ```
    pub async fn list_roles(
        &self,
        request: RoleListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<RoleListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_ROLES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }
    /// 获取用户角色列表
    ///
    /// 该接口用于获取用户的角色分配情况，可以查看
    /// 特定用户拥有的角色权限。
    ///
    /// # 参数
    ///
    /// - `user_id`: 用户ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let user_id = "user_123456";
    /// let response = client.hire.recruitment_config.auth.get_user_roles(user_id, None).await?;
    /// ```
    pub async fn get_user_roles(
        &self,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UserRoleListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_USER_ROLES, "user_id", user_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}
