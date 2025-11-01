//! 用户管理服务
//!
//! 提供完整的用户生命周期管理功能，包括：
//! - 创建、更新、删除用户
//! - 获取用户信息（单个/批量）
//! - 搜索用户
//! - 部门用户查询
//! - 恢复已删除用户
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::contact::v3::user::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 创建新用户
//!     let user = User {
//!         name: Some("张三".to_string()),
//!         mobile: Some("+8613800138000".to_string()),
//!         email: Some("zhangsan@example.com".to_string()),
//!         department_ids: Some(vec!["dept_id".to_string()]),
//!         ..Default::default()
//!     };
//!
//!     let response = client.contact.v3.user
//!         .create_user_builder()
//!         .user(user)
//!         .user_id_type("open_id")
//!         .department_id_type("department_id")
//!         .execute(&client.contact.v3.user)
//!         .await?;
//!
//!     println!("用户创建成功，用户ID: {}", response.data.user.user_id.unwrap_or_default());
//!     Ok(())
//! }
//! ```

use open_lark_core::core::api_req::ApiRequest; // trait_system::ExecutableBuilder temporarily disabled
use crate::core::{
    api_resp::BaseResponse,
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult
};
use crate::service::contact::models::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 用户管理服务
///
/// 提供完整的用户生命周期管理功能，包括用户的创建、更新、删除、查询等操作。
/// 支持企业级的用户管理需求，包括批量操作和高级搜索功能。
#[derive(Debug, Clone)]
pub struct UserService {
    pub config: Config,
}

impl UserService {
    /// 创建用户服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::UserService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = UserService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建用户
    ///
    /// 创建新的用户账号，支持设置用户的基本信息、部门归属等。
    /// 创建成功后，用户将可以使用邮箱或手机号登录飞书系统。
    ///
    /// # API文档
    ///
    /// 创建新的用户账号，系统会自动分配用户ID。
    /// 支持设置用户的姓名、邮箱、手机号、部门等基本信息。
    ///
    /// # 参数
    ///
    /// * `request` - 创建用户的请求参数，包含用户信息和相关配置
    ///
    /// # 返回值
    ///
    /// 返回创建成功的用户信息，包含系统分配的用户ID
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let user = User {
    ///     name: Some("张三".to_string()),
    ///     mobile: Some("+8613800138000".to_string()),
    ///     email: Some("zhangsan@example.com".to_string()),
    ///     department_ids: Some(vec!["dept_001".to_string()]),
    ///     ..Default::default()
    /// };
    ///
    /// let request = CreateUserRequest {
    ///     user,
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("department_id".to_string()),
    /// };
    ///
    /// let response = client.contact.v3.user.create(&request).await?;
    /// println!("用户创建成功，用户ID: {}", response.data.user.user_id.unwrap_or_default());
    /// ```
    pub async fn create(
        &self,
        request: &CreateUserRequest,
    ) -> SDKResult<BaseResponse<CreateUserResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/users".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 修改用户部分信息
    ///
    /// 修改用户的部分信息，支持更新用户的基本属性、部门归属等。
    /// 只更新传入的字段，未传入的字段保持不变。
    ///
    /// # API文档
    ///
    /// 修改用户的部分信息，只更新提供的字段。
    /// 支持修改用户的姓名、邮箱、手机号、部门等信息。
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 修改用户的请求参数
    ///
    /// # 返回值
    ///
    /// 返回修改后的用户信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let user = User {
    ///     name: Some("张三(更新)".to_string()),
    ///     mobile: Some("+8613800138001".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = PatchUserRequest {
    ///     user,
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("department_id".to_string()),
    /// };
    ///
    /// let response = client.contact.v3.user.patch("user_001", &request).await?;
    /// println!("用户修改成功");
    /// ```
    pub async fn patch(
        &self,
        user_id: &str,
        request: &PatchUserRequest,
    ) -> SDKResult<BaseResponse<PatchUserResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: format!("/open-apis/contact/v3/users/{}", user_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取单个用户信息
    ///
    /// 根据用户ID获取用户的详细信息，包括基本信息、部门信息、工作信息等。
    /// 支持使用不同的用户ID类型进行查询。
    ///
    /// # API文档
    ///
    /// 根据用户ID获取用户的详细信息。
    /// 返回用户的基本信息、部门归属、职位信息等完整资料。
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 获取用户的请求参数，包含ID类型配置
    ///
    /// # 返回值
    ///
    /// 返回用户的详细信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let request = GetUserRequest {
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("department_id".to_string()),
    /// };
    ///
    /// let response = client.contact.v3.user.get("user_001", &request).await?;
    /// println!("用户姓名: {}", response.data.user.name.unwrap_or_default());
    /// ```
    pub async fn get(
        &self,
        user_id: &str,
        request: &GetUserRequest,
    ) -> SDKResult<BaseResponse<GetUserResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.clone());
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.insert("department_id_type".to_string(), department_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/contact/v3/users/{}", user_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 批量获取用户信息
    ///
    /// 根据用户ID列表批量获取多个用户的详细信息。
    /// 适用于需要同时查询多个用户信息的场景。
    ///
    /// # API文档
    ///
    /// 根据用户ID列表批量获取用户信息。
    /// 最多支持50个用户ID的批量查询。
    ///
    /// # 参数
    ///
    /// * `request` - 批量获取用户的请求参数
    ///
    /// # 返回值
    ///
    /// 返回批量用户的详细信息列表
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let request = BatchGetUsersRequest {
    ///     user_ids: vec!["user_001".to_string(), "user_002".to_string()],
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("department_id".to_string()),
    /// };
    ///
    /// let response = client.contact.v3.user.batch(&request).await?;
    /// println!("获取到 {} 个用户", response.data.items.len());
    /// ```
    pub async fn batch(
        &self,
        request: &BatchGetUsersRequest,
    ) -> SDKResult<BaseResponse<BatchGetUsersResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/users/batch_get_id".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取用户列表
    ///
    /// 获取用户列表，支持分页查询和多种过滤条件。
    /// 可以按部门、用户状态等条件进行筛选。
    ///
    /// # API文档
    ///
    /// 获取用户列表，支持分页查询。
    /// 返回用户的基本信息列表，支持分页和过滤。
    ///
    /// # 参数
    ///
    /// * `request` - 获取用户列表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回用户列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let request = ListUsersRequest {
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("department_id".to_string()),
    /// };
    ///
    /// let response = client.contact.v3.user.list(&request).await?;
    /// println!("获取到 {} 个用户", response.data.items.len());
    /// ```
    pub async fn list(
        &self,
        request: &ListUsersRequest,
    ) -> SDKResult<BaseResponse<ListUsersResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token".to_string(), page_token.clone());
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.clone());
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.insert("department_id_type".to_string(), department_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/contact/v3/users".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 删除用户
    ///
    /// 删除指定的用户账号，删除后用户将无法登录系统。
    /// 删除操作不可逆，请谨慎使用。
    ///
    /// # API文档
    ///
    /// 删除用户账号，操作不可逆。
    /// 删除后用户将无法登录，相关数据将被清理。
    ///
    /// # 参数
    ///
    /// * `user_id` - 要删除的用户ID
    /// * `request` - 删除用户的请求参数
    ///
    /// # 返回值
    ///
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let request = DeleteUserRequest {
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("department_id".to_string()),
    /// };
    ///
    /// let response = client.contact.v3.user.delete("user_001", &request).await?;
    /// println!("用户删除成功");
    /// ```
    pub async fn delete(
        &self,
        user_id: &str,
        request: &DeleteUserRequest,
    ) -> SDKResult<BaseResponse<DeleteUserResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.clone());
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.insert("department_id_type".to_string(), department_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!("/open-apis/contact/v3/users/{}", user_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, self.config.clone(), None).await?;
        Ok(api_resp)
    }

    /// 搜索用户
    ///
    /// 根据关键词搜索用户，支持按姓名、邮箱、手机号等字段进行搜索。
    /// 返回匹配的用户列表，支持分页查询。
    ///
    /// # API文档
    ///
    /// 根据关键词搜索用户。
    /// 支持按姓名、邮箱、手机号等多种字段进行模糊搜索。
    ///
    /// # 参数
    ///
    /// * `request` - 搜索用户的请求参数
    ///
    /// # 返回值
    ///
    /// 返回搜索结果和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let request = SearchUsersRequest {
    ///     query: "张三".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("department_id".to_string()),
    /// };
    ///
    /// let response = client.contact.v3.user.search(&request).await?;
    /// println!("搜索到 {} 个用户", response.data.items.len());
    /// ```
    pub async fn search(
        &self,
        request: &SearchUsersRequest,
    ) -> SDKResult<BaseResponse<SearchUsersResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/users/search".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    // ==================== Builder模式实现 ====================

    /// 创建用户构建器
    ///
    /// 提供流式API来构建创建用户的请求参数。
    /// 支持链式调用，方便构建复杂的用户创建请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let builder = CreateUserBuilder::new()
    ///     .user(user)
    ///     .user_id_type("open_id")
    ///     .department_id_type("department_id");
    ///
    /// let request = builder.build();
    /// ```
    pub fn create_user_builder(&self) -> CreateUserBuilder {
        CreateUserBuilder::new()
    }

    /// 修改用户构建器
    ///
    /// 提供流式API来构建修改用户的请求参数。
    /// 支持链式调用，方便构建用户修改请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let builder = PatchUserBuilder::new("user_001")
    ///     .user(user)
    ///     .user_id_type("open_id")
    ///     .department_id_type("department_id");
    ///
    /// let request = builder.build();
    /// ```
    pub fn patch_user_builder(&self, user_id: &str) -> PatchUserBuilder {
        PatchUserBuilder::new(user_id)
    }

    /// 获取用户构建器
    ///
    /// 提供流式API来构建获取用户的请求参数。
    /// 支持链式调用，方便配置查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let builder = GetUserBuilder::new("user_001")
    ///     .user_id_type("open_id")
    ///     .department_id_type("department_id");
    ///
    /// let request = builder.build();
    /// ```
    pub fn get_user_builder(&self, user_id: &str) -> GetUserBuilder {
        GetUserBuilder::new(user_id)
    }

    /// 批量获取用户构建器
    ///
    /// 提供流式API来构建批量获取用户的请求参数。
    /// 支持链式调用，方便配置批量查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let builder = BatchGetUsersBuilder::new()
    ///     .user_ids(vec!["user_001".to_string(), "user_002".to_string()])
    ///     .user_id_type("open_id")
    ///     .department_id_type("department_id");
    ///
    /// let request = builder.build();
    /// ```
    pub fn batch_get_users_builder(&self) -> BatchGetUsersBuilder {
        BatchGetUsersBuilder::new()
    }

    /// 获取用户列表构建器
    ///
    /// 提供流式API来构建获取用户列表的请求参数。
    /// 支持链式调用，方便配置分页和过滤参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let builder = ListUsersBuilder::new()
    ///     .page_size(20)
    ///     .user_id_type("open_id")
    ///     .department_id_type("department_id");
    ///
    /// let request = builder.build();
    /// ```
    pub fn list_users_builder(&self) -> ListUsersBuilder {
        ListUsersBuilder::new()
    }

    /// 删除用户构建器
    ///
    /// 提供流式API来构建删除用户的请求参数。
    /// 支持链式调用，方便配置删除参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let builder = DeleteUserBuilder::new("user_001")
    ///     .user_id_type("open_id")
    ///     .department_id_type("department_id");
    ///
    /// let request = builder.build();
    /// ```
    pub fn delete_user_builder(&self, user_id: &str) -> DeleteUserBuilder {
        DeleteUserBuilder::new(user_id)
    }

    /// 搜索用户构建器
    ///
    /// 提供流式API来构建搜索用户的请求参数。
    /// 支持链式调用，方便配置搜索条件。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::user::*;
    ///
    /// let builder = SearchUsersBuilder::new()
    ///     .query("张三".to_string())
    ///     .page_size(20)
    ///     .user_id_type("open_id")
    ///     .department_id_type("department_id");
    ///
    /// let request = builder.build();
    /// ```
    pub fn search_users_builder(&self) -> SearchUsersBuilder {
        SearchUsersBuilder::new()
    }
}

// ==================== Builder结构体实现 ====================

/// 创建用户构建器
#[derive(Debug, Clone)]
pub struct CreateUserBuilder {
    request: CreateUserRequest,
}

impl CreateUserBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: CreateUserRequest::default(),
        }
    }

    /// 设置用户信息
    pub fn user(mut self, user: User) -> Self {
        self.request.user = user;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreateUserRequest {
        self.request
    }
}

impl Default for CreateUserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    CreateUserBuilder,
//    UserService,
//    CreateUserRequest,
//    BaseResponse<CreateUserResponse>,
//    create
//);

/// 修改用户构建器
#[derive(Debug, Clone)]
pub struct PatchUserBuilder {
    user_id: String,
    request: PatchUserRequest,
}

impl PatchUserBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: PatchUserRequest::default(),
        }
    }

    /// 设置用户信息
    pub fn user(mut self, user: User) -> Self {
        self.request.user = user;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> PatchUserRequest {
        self.request
    }
}

impl Default for PatchUserBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    PatchUserBuilder,
//    UserService,
//    PatchUserRequest,
//    BaseResponse<PatchUserResponse>,
//    patch
//);

/// 获取用户构建器
#[derive(Debug, Clone)]
pub struct GetUserBuilder {
    user_id: String,
    request: GetUserRequest,
}

impl GetUserBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: GetUserRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetUserRequest {
        self.request
    }
}

impl Default for GetUserBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetUserBuilder,
//    UserService,
//    GetUserRequest,
//    BaseResponse<GetUserResponse>,
//    get
//);

/// 批量获取用户构建器
#[derive(Debug, Clone)]
pub struct BatchGetUsersBuilder {
    request: BatchGetUsersRequest,
}

impl BatchGetUsersBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: BatchGetUsersRequest::default(),
        }
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.request.user_ids = user_ids;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> BatchGetUsersRequest {
        self.request
    }
}

impl Default for BatchGetUsersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    BatchGetUsersBuilder,
//    UserService,
//    BatchGetUsersRequest,
//    BaseResponse<BatchGetUsersResponse>,
//    batch
//);

/// 获取用户列表构建器
#[derive(Debug, Clone)]
pub struct ListUsersBuilder {
    request: ListUsersRequest,
}

impl ListUsersBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: ListUsersRequest::default(),
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> ListUsersRequest {
        self.request
    }
}

impl Default for ListUsersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    ListUsersBuilder,
//    UserService,
//    ListUsersRequest,
//    BaseResponse<ListUsersResponse>,
//    list
//);

/// 删除用户构建器
#[derive(Debug, Clone)]
pub struct DeleteUserBuilder {
    user_id: String,
    request: DeleteUserRequest,
}

impl DeleteUserBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: DeleteUserRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> DeleteUserRequest {
        self.request
    }
}

impl Default for DeleteUserBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    DeleteUserBuilder,
//    UserService,
//    DeleteUserRequest,
//    BaseResponse<DeleteUserResponse>,
//    delete
//);

/// 搜索用户构建器
#[derive(Debug, Clone)]
pub struct SearchUsersBuilder {
    request: SearchUsersRequest,
}

impl SearchUsersBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchUsersRequest::default(),
        }
    }

    /// 设置搜索关键词
    pub fn query(mut self, query: String) -> Self {
        self.request.query = query;
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> SearchUsersRequest {
        self.request
    }
}

impl Default for SearchUsersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchUsersBuilder,
//    UserService,
//    SearchUsersRequest,
//    BaseResponse<SearchUsersResponse>,
//    search
//);

// ==================== 数据模型 ====================

/// 创建用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// 用户信息
    pub user: User,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for CreateUserRequest {
    fn default() -> Self {
        Self {
            user: User::default(),
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 创建用户响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponse {
    /// 用户信息
    pub user: User,
}

/// 修改用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchUserRequest {
    /// 用户信息
    pub user: User,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for PatchUserRequest {
    fn default() -> Self {
        Self {
            user: User::default(),
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 修改用户响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchUserResponse {
    /// 用户信息
    pub user: User,
}

/// 获取用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for GetUserRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 获取用户响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserResponse {
    /// 用户信息
    pub user: User,
}

/// 批量获取用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUsersRequest {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for BatchGetUsersRequest {
    fn default() -> Self {
        Self {
            user_ids: vec![],
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 批量获取用户响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUsersResponse {
    /// 用户列表
    pub items: Vec<User>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取用户列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for ListUsersRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 获取用户列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersResponse {
    /// 用户列表
    pub items: Vec<User>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 删除用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for DeleteUserRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 删除用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteUserResponse {}

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
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for SearchUsersRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 搜索用户响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersResponse {
    /// 用户列表
    pub items: Vec<User>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
}