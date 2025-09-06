use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, http::Transport, req_option::RequestOption,
        standard_response::StandardResponse, trait_system::executable_builder::ExecutableBuilder,
        SDKResult,
    },
    service::contact::models::*,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 用户管理服务
///
/// 提供完整的用户生命周期管理功能，包括：
/// - 创建、更新、删除用户
/// - 获取用户信息（单个/批量）
/// - 搜索用户
/// - 部门用户查询
/// - 恢复已删除用户
pub struct UserService {
    config: Config,
}

impl UserService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建用户
    pub async fn create(
        &self,
        req: &CreateUserRequest,
    ) -> crate::core::SDKResult<CreateUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/users".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateUserResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 修改用户部分信息
    pub async fn patch(
        &self,
        user_id: &str,
        req: &PatchUserRequest,
    ) -> crate::core::SDKResult<PatchUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: format!("/open-apis/contact/v3/users/{user_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<PatchUserResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 更新用户 ID
    pub async fn update_user_id(
        &self,
        user_id: &str,
        req: &UpdateUserIdRequest,
    ) -> crate::core::SDKResult<UpdateUserIdResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: format!("/open-apis/contact/v3/users/{user_id}/update_user_id"),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<UpdateUserIdResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 获取单个用户信息
    pub async fn get(
        &self,
        user_id: &str,
        _req: &GetUserRequest,
    ) -> crate::core::SDKResult<GetUserResponse> {
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &_req.user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.clone());
        }
        if let Some(department_id_type) = &_req.department_id_type {
            query_params.insert("department_id_type".to_string(), department_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/contact/v3/users/{user_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let resp = Transport::<GetUserResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 批量获取用户信息
    pub async fn batch(
        &self,
        req: &BatchGetUsersRequest,
    ) -> crate::core::SDKResult<BatchGetUsersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/users/batch".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<BatchGetUsersResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 获取部门直属用户列表
    pub async fn find_by_department(
        &self,
        _req: &FindUsersByDepartmentRequest,
    ) -> crate::core::SDKResult<FindUsersByDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/contact/v3/users/find_by_department".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<FindUsersByDepartmentResponse>::request(api_req, &self.config, None)
            .await?;
        resp.into_result()
    }

    /// 通过手机号或邮箱获取用户 ID
    pub async fn batch_get_id(
        &self,
        req: &BatchGetUserIdRequest,
    ) -> crate::core::SDKResult<BatchGetUserIdResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/users/batch_get_id".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchGetUserIdResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 搜索用户
    pub async fn search(
        &self,
        req: &SearchUsersRequest,
    ) -> crate::core::SDKResult<SearchUsersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/users/search".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<SearchUsersResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 删除用户
    pub async fn delete(
        &self,
        user_id: &str,
        _req: &DeleteUserRequest,
    ) -> crate::core::SDKResult<DeleteUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!("/open-apis/contact/v3/users/{user_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteUserResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 恢复已删除用户
    pub async fn resurrect(
        &self,
        user_id: &str,
        req: &ResurrectUserRequest,
    ) -> crate::core::SDKResult<ResurrectUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!("/open-apis/contact/v3/users/{user_id}/resurrect"),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<ResurrectUserResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 获取用户列表
    pub async fn list(&self, req: &ListUsersRequest) -> crate::core::SDKResult<ListUsersResponse> {
        let mut query_params = std::collections::HashMap::new();

        if let Some(page_size) = req.page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = &req.page_token {
            query_params.insert("page_token".to_string(), page_token.clone());
        }
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.clone());
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.insert("department_id_type".to_string(), department_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/contact/v3/users".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let resp = Transport::<ListUsersResponse>::request(api_req, &self.config, None).await?;
        resp.into_result()
    }

    /// 创建用户 - Builder模式 (推荐)
    ///
    /// 提供更现代化的Builder接口，支持链式调用和统一的执行模式
    pub fn create_user_builder(&self) -> CreateUserBuilder {
        CreateUserBuilder::new()
    }
}

/// 创建用户的Builder
#[derive(Default)]
pub struct CreateUserBuilder {
    user: Option<User>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
}

impl CreateUserBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户信息
    pub fn user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: impl ToString) -> Self {
        self.department_id_type = Some(department_id_type.to_string());
        self
    }

    pub fn build(self) -> CreateUserRequest {
        CreateUserRequest {
            user: self.user.unwrap_or_default(),
            user_id_type: self.user_id_type,
            department_id_type: self.department_id_type,
        }
    }
}

#[async_trait]
impl ExecutableBuilder<UserService, CreateUserRequest, CreateUserResponse> for CreateUserBuilder {
    fn build(self) -> CreateUserRequest {
        self.build()
    }

    async fn execute(self, service: &UserService) -> SDKResult<CreateUserResponse> {
        let req = self.build();
        service.create(&req).await
    }

    async fn execute_with_options(
        self,
        service: &UserService,
        _option: RequestOption,
    ) -> SDKResult<CreateUserResponse> {
        // 目前简单实现，后续可以支持传递option到service方法
        let req = self.build();
        service.create(&req).await
    }
}

// 请求/响应结构体定义

/// 创建用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// 用户信息
    pub user: User,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 创建用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for CreateUserResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 修改用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchUserRequest {
    /// 用户信息
    pub user: User,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 修改用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for PatchUserResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 更新用户ID请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserIdRequest {
    /// 新的用户ID
    pub new_user_id: String,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 更新用户ID响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateUserIdResponse {}

impl ApiResponseTrait for UpdateUserIdResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取用户请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserRequest {
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 获取用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for GetUserResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 批量获取用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUsersRequest {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 批量获取用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetUsersResponse {
    /// 用户列表
    pub items: Vec<User>,
}

impl ApiResponseTrait for BatchGetUsersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 按部门查找用户请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FindUsersByDepartmentRequest {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 按部门查找用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FindUsersByDepartmentResponse {
    /// 用户列表
    pub items: Vec<User>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for FindUsersByDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 批量获取用户ID请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUserIdRequest {
    /// 邮箱列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// 手机号列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobiles: Option<Vec<String>>,
    /// 包含已离职用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resigned: Option<bool>,
}

/// 批量获取用户ID响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetUserIdResponse {
    /// 用户列表
    pub user_list: Vec<UserIdInfo>,
}

impl ApiResponseTrait for BatchGetUserIdResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 用户ID信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdInfo {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}

/// 搜索用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersRequest {
    /// 搜索关键词
    pub query: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 搜索用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchUsersResponse {
    /// 用户列表
    pub items: Vec<User>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchUsersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 删除用户请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 删除用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteUserResponse {}

impl ApiResponseTrait for DeleteUserResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 恢复用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResurrectUserRequest {
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 恢复用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResurrectUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for ResurrectUserResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取用户列表请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUsersRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 获取用户列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListUsersResponse {
    /// 用户列表
    pub items: Vec<User>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListUsersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}
