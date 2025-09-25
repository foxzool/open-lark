use crate::impl_full_service;
use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
        req_option::RequestOption, standard_response::StandardResponse,
        trait_system::executable_builder::ExecutableBuilder, SDKResult,
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

// Service 抽象接入（标准样例）：Contact v3 UserService
// 要求结构体包含 `config: Config` 字段
impl_full_service!(UserService, "contact.user", "v3");

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
            api_path: crate::core::endpoints::contact::CONTACT_V3_USERS.to_string(),
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
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_USER_GET,
                "user_id",
                user_id,
            ),
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
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_USER_UPDATE_ID,
                "user_id",
                user_id,
            ),
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
            query_params.insert("user_id_type", user_id_type.clone());
        }
        if let Some(department_id_type) = &_req.department_id_type {
            query_params.insert("department_id_type", department_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_USER_GET,
                "user_id",
                user_id,
            ),
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
            api_path: crate::core::endpoints::contact::CONTACT_V3_USERS_BATCH.to_string(),
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
            api_path: crate::core::endpoints::contact::CONTACT_V3_USERS_FIND_BY_DEPARTMENT
                .to_string(),
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
            api_path: crate::core::endpoints::contact::CONTACT_V3_USERS_BATCH_GET_ID.to_string(),
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
            api_path: crate::core::endpoints::contact::CONTACT_V3_USERS_SEARCH.to_string(),
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
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_USER_GET,
                "user_id",
                user_id,
            ),
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
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_USER_RESURRECT,
                "user_id",
                user_id,
            ),
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
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &req.page_token {
            query_params.insert("page_token", page_token.clone());
        }
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.insert("department_id_type", department_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_USERS.to_string(),
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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::{
        core::api_resp::ResponseFormat, core::config::Config, service::contact::models::User,
    };

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.example.com")
            .build()
    }

    fn create_test_user() -> User {
        use crate::service::contact::models::{Avatar, UserCustomAttr, UserStatus};

        User {
            user_id: Some("user123".to_string()),
            name: Some("Test User".to_string()),
            en_name: Some("test_user".to_string()),
            email: Some("test@example.com".to_string()),
            mobile: Some("+86138000000".to_string()),
            mobile_visible: Some(true),
            gender: Some(1),
            avatar: Some(Avatar {
                avatar_72: Some("https://example.com/avatar_72.jpg".to_string()),
                avatar_240: Some("https://example.com/avatar_240.jpg".to_string()),
                avatar_640: Some("https://example.com/avatar_640.jpg".to_string()),
                avatar_origin: Some("https://example.com/avatar_origin.jpg".to_string()),
            }),
            status: Some(UserStatus {
                is_frozen: Some(false),
                is_resigned: Some(false),
                is_activated: Some(true),
                is_exited: Some(false),
                is_unjoin: Some(false),
            }),
            department_ids: Some(vec!["dept1".to_string(), "dept2".to_string()]),
            leader_user_id: Some("leader123".to_string()),
            city: Some("Beijing".to_string()),
            country: Some("China".to_string()),
            work_station: Some("Workstation 101".to_string()),
            join_time: Some(1634567890),
            employee_no: Some("EMP001".to_string()),
            employee_type: Some(1),
            custom_attrs: Some(vec![UserCustomAttr {
                r#type: Some("text".to_string()),
                id: Some("custom_1".to_string()),
                value: Some(serde_json::Value::String("test_value".to_string())),
            }]),
            enterprise_email: Some("test@company.com".to_string()),
            job_title: Some("Engineer".to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn test_user_service_new() {
        let config = create_test_config();
        let service = UserService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
        assert_eq!(service.config.base_url, config.base_url);
    }

    #[test]
    fn test_create_user_request_serialization() {
        let user = create_test_user();
        let request = CreateUserRequest {
            user: user.clone(),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test@example.com"));
        assert!(json.contains("Test User"));
        assert!(json.contains("open_id"));
    }

    #[test]
    fn test_create_user_response_deserialization() {
        let json = r#"{
            "user": {
                "user_id": "user123",
                "name": "Test User",
                "email": "test@example.com",
                "mobile": "+86138000000"
            }
        }"#;

        let response: CreateUserResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.user.user_id, Some("user123".to_string()));
        assert_eq!(response.user.name, Some("Test User".to_string()));
        assert_eq!(response.user.email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_create_user_builder() {
        let user = create_test_user();
        let builder = CreateUserBuilder::new()
            .user(user.clone())
            .user_id_type("open_id")
            .department_id_type("department_id");

        let request = builder.build();
        assert_eq!(request.user.name, user.name);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_create_user_builder_default() {
        let builder = CreateUserBuilder::new();
        let request = builder.build();

        assert_eq!(request.user.name, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_patch_user_request_serialization() {
        let user = create_test_user();
        let request = PatchUserRequest {
            user: user.clone(),
            user_id_type: Some("user_id".to_string()),
            department_id_type: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Test User"));
        assert!(json.contains("user_id"));
        assert!(!json.contains("department_id_type"));
    }

    #[test]
    fn test_update_user_id_request_serialization() {
        let request = UpdateUserIdRequest {
            new_user_id: "new_user_123".to_string(),
            user_id_type: Some("open_id".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("new_user_123"));
        assert!(json.contains("open_id"));
    }

    #[test]
    fn test_get_user_request_default() {
        let request = GetUserRequest::default();
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_batch_get_users_request() {
        let request = BatchGetUsersRequest {
            user_ids: vec![
                "user1".to_string(),
                "user2".to_string(),
                "user3".to_string(),
            ],
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("user1"));
        assert!(json.contains("user2"));
        assert!(json.contains("user3"));
        assert_eq!(request.user_ids.len(), 3);
    }

    #[test]
    fn test_batch_get_users_response() {
        let user1 = create_test_user();
        let mut user2 = create_test_user();
        user2.user_id = Some("user456".to_string());
        user2.name = Some("Another User".to_string());

        let response = BatchGetUsersResponse {
            items: vec![user1, user2],
        };

        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].user_id, Some("user123".to_string()));
        assert_eq!(response.items[1].user_id, Some("user456".to_string()));
    }

    #[test]
    fn test_find_users_by_department_request() {
        let request = FindUsersByDepartmentRequest {
            department_id: Some("dept123".to_string()),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("dept123"));
        assert!(json.contains("50"));
        assert!(json.contains("token123"));
    }

    #[test]
    fn test_find_users_by_department_response() {
        let user = create_test_user();
        let response = FindUsersByDepartmentResponse {
            items: vec![user],
            has_more: Some(true),
            page_token: Some("next_token".to_string()),
        };

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token, Some("next_token".to_string()));
    }

    #[test]
    fn test_batch_get_user_id_request() {
        let request = BatchGetUserIdRequest {
            emails: Some(vec![
                "test1@example.com".to_string(),
                "test2@example.com".to_string(),
            ]),
            mobiles: Some(vec!["+86138000001".to_string(), "+86138000002".to_string()]),
            include_resigned: Some(true),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test1@example.com"));
        assert!(json.contains("+86138000001"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_user_id_info_serialization() {
        let user_info = UserIdInfo {
            user_id: Some("user123".to_string()),
            email: Some("test@example.com".to_string()),
            mobile: Some("+86138000000".to_string()),
        };

        let json = serde_json::to_string(&user_info).unwrap();
        let deserialized: UserIdInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.user_id, user_info.user_id);
        assert_eq!(deserialized.email, user_info.email);
        assert_eq!(deserialized.mobile, user_info.mobile);
    }

    #[test]
    fn test_search_users_request() {
        let request = SearchUsersRequest {
            query: "张三".to_string(),
            page_size: Some(20),
            page_token: Some("search_token".to_string()),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("张三"));
        assert!(json.contains("20"));
        assert!(json.contains("search_token"));
    }

    #[test]
    fn test_search_users_response() {
        let user = create_test_user();
        let response = SearchUsersResponse {
            items: vec![user],
            has_more: Some(false),
            page_token: None,
        };

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.has_more, Some(false));
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_delete_user_request_default() {
        let request = DeleteUserRequest::default();
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_resurrect_user_request() {
        let request = ResurrectUserRequest {
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("open_id"));
        assert!(json.contains("department_id"));
    }

    #[test]
    fn test_list_users_request() {
        let request = ListUsersRequest {
            page_size: Some(100),
            page_token: Some("list_token".to_string()),
            user_id_type: Some("user_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("100"));
        assert!(json.contains("list_token"));
        assert!(json.contains("user_id"));
        assert!(json.contains("department_id"));
    }

    #[test]
    fn test_list_users_response() {
        let user1 = create_test_user();
        let mut user2 = create_test_user();
        user2.user_id = Some("user789".to_string());
        user2.name = Some("Third User".to_string());

        let response = ListUsersResponse {
            items: vec![user1, user2],
            has_more: Some(true),
            page_token: Some("next_page_token".to_string()),
        };

        assert_eq!(response.items.len(), 2);
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementations() {
        assert!(matches!(
            CreateUserResponse::data_format(),
            ResponseFormat::Data
        ));
        assert!(matches!(
            PatchUserResponse::data_format(),
            ResponseFormat::Data
        ));
        assert!(matches!(
            UpdateUserIdResponse::data_format(),
            ResponseFormat::Data
        ));
        assert!(matches!(
            GetUserResponse::data_format(),
            ResponseFormat::Data
        ));
        assert!(matches!(
            BatchGetUsersResponse::data_format(),
            ResponseFormat::Data
        ));
    }

    #[test]
    fn test_empty_responses() {
        let update_response = UpdateUserIdResponse {};
        let delete_response = DeleteUserResponse {};

        let update_json = serde_json::to_string(&update_response).unwrap();
        let delete_json = serde_json::to_string(&delete_response).unwrap();

        assert_eq!(update_json, "{}");
        assert_eq!(delete_json, "{}");
    }

    #[test]
    fn test_user_service_builder_creation() {
        let config = create_test_config();
        let service = UserService::new(config);
        let builder = service.create_user_builder();

        let user = create_test_user();
        let request = builder
            .user(user.clone())
            .user_id_type("open_id")
            .department_id_type("department_id")
            .build();

        assert_eq!(request.user.name, user.name);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_request_serialization_edge_cases() {
        let request = CreateUserRequest {
            user: User::default(),
            user_id_type: None,
            department_id_type: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateUserRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.user_id_type, None);
        assert_eq!(deserialized.department_id_type, None);
    }

    #[test]
    fn test_unicode_handling() {
        let mut user = create_test_user();
        user.name = Some("张三".to_string());
        user.city = Some("北京市".to_string());
        user.country = Some("中国".to_string());

        let request = CreateUserRequest {
            user: user.clone(),
            user_id_type: Some("用户ID".to_string()),
            department_id_type: Some("部门ID".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateUserRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.user.name, Some("张三".to_string()));
        assert_eq!(deserialized.user.city, Some("北京市".to_string()));
        assert_eq!(deserialized.user.country, Some("中国".to_string()));
        assert_eq!(deserialized.user_id_type, Some("用户ID".to_string()));
        assert_eq!(deserialized.department_id_type, Some("部门ID".to_string()));
    }
}
