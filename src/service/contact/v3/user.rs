#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 用户管理服务
//!
//! 提供完整的用户生命周期管理功能：
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

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户union_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// 用户open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户英文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 用户昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 用户头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 用户头像72x72
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72x72: Option<String>,
    /// 用户头像240x240
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240x240: Option<String>,
    /// 用户头像640x640
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640x640: Option<String>,
    /// 用户头像original
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_original: Option<String>,
    /// 用户性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<UserGender>,
    /// 用户员工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 用户职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 用户电话分机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    /// 用户工作地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_station: Option<String>,
    /// 用户所属部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 用户所属部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<UserDepartment>>,
    /// 用户直接上级ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<Vec<String>>,
    /// 用户排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<UserOrder>>,
    /// 用户自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<UserCustomAttribute>>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 用户是否是HR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hrm: Option<bool>,
    /// 用户角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<UserRole>>,
    /// 用户扩展属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_extensions: Option<UserExtensions>,
    /// 用户入职时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<i32>,
    /// 钱包地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            user_id: None,
            union_id: None,
            open_id: None,
            name: None,
            en_name: None,
            nickname: None,
            email: None,
            mobile: None,
            avatar: None,
            avatar_72x72: None,
            avatar_240x240: None,
            avatar_640x640: None,
            avatar_original: None,
            gender: None,
            employee_no: None,
            position: None,
            telephone: None,
            work_station: None,
            department_ids: None,
            departments: None,
            leader_user_id: None,
            orders: None,
            custom_attributes: None,
            status: None,
            is_hrm: None,
            roles: None,
            user_extensions: None,
            employee_type: None,
            wallet_address: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 用户性别枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserGender {
    /// 男性
    Male,
    /// 女性
    Female,
    /// 未知
    Unknown,
}

impl Default for UserGender {
    fn default() -> Self {
        Self::Unknown
    }
}

/// 用户状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    /// 激活
    Active,
    /// 未激活
    Inactive,
    /// 禁用
    Disabled,
    /// 已删除
    Deleted,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Inactive
    }
}

/// 用户所属部门信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDepartment {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 部门路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_path: Option<String>,
}

impl Default for UserDepartment {
    fn default() -> Self {
        Self {
            department_id: None,
            name: None,
            department_path: None,
        }
    }
}

/// 用户排序信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserOrder {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 排序值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

impl Default for UserOrder {
    fn default() -> Self {
        Self {
            department_id: None,
            order: None,
        }
    }
}

/// 用户自定义属性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserCustomAttribute {
    /// 属性ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 属性Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 属性值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl Default for UserCustomAttribute {
    fn default() -> Self {
        Self {
            id: None,
            key: None,
            value: None,
        }
    }
}

/// 用户角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserRole {
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 角色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
}

impl Default for UserRole {
    fn default() -> Self {
        Self {
            name: None,
            role_type: None,
        }
    }
}

/// 用户扩展属性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserExtensions {
    /// 扩展属性字典
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}

impl Default for UserExtensions {
    fn default() -> Self {
        Self { extra: None }
    }
}

/// 用户管理服务
#[derive(Debug, Clone)]
pub struct UserService {
    config: Config,
}

impl UserService {
    /// 创建新的用户服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建用户
    ///
    /// 创建新的用户账号，支持设置用户的基本信息、部门归属等。
    /// 创建成功后，用户将可以使用邮箱或手机号登录飞书系统。
    ///
    /// # 参数
    /// * `req` - 创建用户请求
    ///
    /// # 返回值
    /// 返回创建成功的用户详细信息
    pub async fn create(&self, req: &CreateUserRequest) -> SDKResult<CreateUserResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::CONTACT_V3_USERS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 修改用户部分信息
    ///
    /// 部分更新用户信息（不覆盖未提供的字段）
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `req` - 修改用户请求
    ///
    /// # 返回值
    /// 返回修改后的用户详细信息
    pub async fn patch(
        &self,
        user_id: &str,
        req: &PatchUserRequest,
    ) -> SDKResult<PatchUserResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_USER_GET
            .replace("{user_id}", user_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<PatchUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新用户所有信息
    ///
    /// 完全更新用户信息（覆盖所有字段）
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `req` - 更新用户请求
    ///
    /// # 返回值
    /// 返回更新后的用户详细信息
    pub async fn update(
        &self,
        user_id: &str,
        req: &UpdateUserRequest,
    ) -> SDKResult<UpdateUserResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_USER_GET
            .replace("{user_id}", user_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<UpdateUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取用户信息
    ///
    /// 根据用户ID获取用户的详细信息
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `req` - 获取用户信息请求
    ///
    /// # 返回值
    /// 返回用户的详细信息
    pub async fn get(&self, user_id: &str, req: &GetUserRequest) -> SDKResult<GetUserResponse> {
        let mut api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_USER_GET
            .replace("{user_id}", user_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除用户
    ///
    /// 删除指定的用户账号
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    ///
    /// # 返回值
    /// 返回删除操作的结果
    pub async fn delete(&self, user_id: &str) -> SDKResult<DeleteUserResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_USER_GET
            .replace("{user_id}", user_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量获取用户信息
    ///
    /// 根据用户ID列表批量获取用户信息
    ///
    /// # 参数
    /// * `req` - 批量获取用户信息请求
    ///
    /// # 返回值
    /// 返回用户信息列表
    pub async fn batch_get(&self, req: &BatchGetUserRequest) -> SDKResult<BatchGetUserResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_USERS_BATCH.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if !req.user_ids.is_empty() {
            query_params.push(format!("user_ids={}", req.user_ids.join(",")));
        }
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<BatchGetUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 搜索用户
    ///
    /// 根据关键词搜索用户
    ///
    /// # 参数
    /// * `req` - 搜索用户请求
    ///
    /// # 返回值
    /// 返回搜索结果，支持分页
    pub async fn search(&self, req: &SearchUserRequest) -> SDKResult<SearchUserResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_USERS_SEARCH.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(query) = &req.query {
            query_params.push(format!("query={}", query));
        }
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<SearchUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 根据部门获取用户列表
    ///
    /// 获取指定部门下的用户列表
    ///
    /// # 参数
    /// * `department_id` - 部门ID
    /// * `req` - 获取部门用户请求
    ///
    /// # 返回值
    /// 返回用户列表，支持分页
    pub async fn get_by_department(
        &self,
        department_id: &str,
        req: &GetUserByDepartmentRequest,
    ) -> SDKResult<GetUserByDepartmentResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_USERS_FIND_BY_DEPARTMENT
                .to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        query_params.push(format!("department_id={}", department_id));
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetUserByDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 恢复已删除的用户
    ///
    /// 恢复之前被删除的用户账号
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    ///
    /// # 返回值
    /// 返回恢复操作的结果
    pub async fn restore(&self, user_id: &str) -> SDKResult<RestoreUserResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_USER_RESURRECT
            .replace("{user_id}", user_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<RestoreUserResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 创建用户请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// 创建用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for CreateUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改用户请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// 修改用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for PatchUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新用户请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateUserRequest {
    /// 用户信息
    pub user: User,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 更新用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for UpdateUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户信息请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetUserRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for GetUserRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 获取用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetUserResponse {
    /// 用户信息
    pub user: User,
}

impl ApiResponseTrait for GetUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteUserResponse {}

impl ApiResponseTrait for DeleteUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取用户信息请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetUserRequest {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 批量获取用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchGetUserResponse {
    /// 用户信息列表
    pub users: Vec<User>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for BatchGetUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索用户请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchUserRequest {
    /// 搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for SearchUserRequest {
    fn default() -> Self {
        Self {
            query: None,
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 搜索用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchUserResponse {
    /// 搜索结果
    pub users: Vec<User>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 根据部门获取用户请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetUserByDepartmentRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for GetUserByDepartmentRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 根据部门获取用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetUserByDepartmentResponse {
    /// 用户列表
    pub users: Vec<User>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetUserByDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 恢复用户响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RestoreUserResponse {}

impl ApiResponseTrait for RestoreUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建用户构建器
#[derive(Debug, Clone)]
pub struct CreateUserBuilder {
    request: CreateUserRequest,
}

impl CreateUserBuilder {
    /// 创建新的用户构建器
    pub fn new() -> Self {
        Self {
            request: CreateUserRequest {
                user: User::default(),
                user_id_type: None,
                department_id_type: None,
            },
        }
    }

    /// 设置用户信息
    pub fn user(mut self, user: User) -> Self {
        self.request.user = user;
        self
    }

    /// 设置用户姓名
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.user.name = Some(name.into());
        self
    }

    /// 设置用户邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.request.user.email = Some(email.into());
        self
    }

    /// 设置用户手机号
    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.request.user.mobile = Some(mobile.into());
        self
    }

    /// 设置部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request.user.department_ids = Some(department_ids);
        self
    }

    /// 设置用户职位
    pub fn position(mut self, position: impl Into<String>) -> Self {
        self.request.user.position = Some(position.into());
        self
    }

    /// 设置用户员工号
    pub fn employee_no(mut self, employee_no: impl Into<String>) -> Self {
        self.request.user.employee_no = Some(employee_no.into());
        self
    }

    /// 设置用户性别
    pub fn gender(mut self, gender: UserGender) -> Self {
        self.request.user.gender = Some(gender);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.request.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行创建操作
    pub async fn execute(self, service: &UserService) -> SDKResult<CreateUserResponse> {
        service.create(&self.request).await
    }
}

impl Default for CreateUserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 搜索用户构建器
#[derive(Debug, Clone)]
pub struct SearchUserBuilder {
    request: SearchUserRequest,
}

impl SearchUserBuilder {
    /// 创建新的搜索构建器
    pub fn new() -> Self {
        Self {
            request: SearchUserRequest::default(),
        }
    }

    /// 设置搜索关键词
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.request.query = Some(query.into());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 执行搜索操作
    pub async fn execute(self, service: &UserService) -> SDKResult<SearchUserResponse> {
        service.search(&self.request).await
    }
}

impl Default for SearchUserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UserService {
    /// 创建用户构建器
    pub fn create_user_builder(&self) -> CreateUserBuilder {
        CreateUserBuilder::new()
    }

    /// 创建搜索用户构建器
    pub fn search_user_builder(&self) -> SearchUserBuilder {
        SearchUserBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_service_creation() {
        let config = Config::default();
        let service = UserService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_user_default_creation() {
        let user = User::default();
        assert_eq!(user.user_id, None);
        assert_eq!(user.union_id, None);
        assert_eq!(user.open_id, None);
        assert_eq!(user.name, None);
        assert_eq!(user.en_name, None);
        assert_eq!(user.nickname, None);
        assert_eq!(user.email, None);
        assert_eq!(user.mobile, None);
        assert_eq!(user.avatar, None);
        assert_eq!(user.gender, None);
        assert_eq!(user.employee_no, None);
        assert_eq!(user.position, None);
        assert_eq!(user.telephone, None);
        assert_eq!(user.work_station, None);
        assert_eq!(user.department_ids, None);
        assert_eq!(user.departments, None);
        assert_eq!(user.leader_user_id, None);
        assert_eq!(user.orders, None);
        assert_eq!(user.custom_attributes, None);
        assert_eq!(user.status, None);
        assert_eq!(user.is_hrm, None);
        assert_eq!(user.roles, None);
        assert_eq!(user.user_extensions, None);
        assert_eq!(user.employee_type, None);
        assert_eq!(user.wallet_address, None);
        assert_eq!(user.create_time, None);
        assert_eq!(user.update_time, None);
    }

    #[test]
    fn test_user_with_data() {
        let user = User {
            user_id: Some("user_123".to_string()),
            union_id: Some("union_456".to_string()),
            open_id: Some("open_789".to_string()),
            name: Some("张三".to_string()),
            en_name: Some("Zhang San".to_string()),
            nickname: Some("小张".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            mobile: Some("+8613800138000".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
            gender: Some(UserGender::Male),
            employee_no: Some("EMP001".to_string()),
            position: Some("软件工程师".to_string()),
            telephone: Some("8888".to_string()),
            work_station: Some("工位A-101".to_string()),
            department_ids: Some(vec!["dept_001".to_string(), "dept_002".to_string()]),
            leader_user_id: Some(vec!["leader_001".to_string()]),
            status: Some(UserStatus::Active),
            is_hrm: Some(false),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-12-31T23:59:59Z".to_string()),
            ..Default::default()
        };

        assert_eq!(user.user_id, Some("user_123".to_string()));
        assert_eq!(user.union_id, Some("union_456".to_string()));
        assert_eq!(user.open_id, Some("open_789".to_string()));
        assert_eq!(user.name, Some("张三".to_string()));
        assert_eq!(user.en_name, Some("Zhang San".to_string()));
        assert_eq!(user.nickname, Some("小张".to_string()));
        assert_eq!(user.email, Some("zhangsan@example.com".to_string()));
        assert_eq!(user.mobile, Some("+8613800138000".to_string()));
        assert_eq!(
            user.avatar,
            Some("https://example.com/avatar.jpg".to_string())
        );
        assert_eq!(user.gender, Some(UserGender::Male));
        assert_eq!(user.employee_no, Some("EMP001".to_string()));
        assert_eq!(user.position, Some("软件工程师".to_string()));
        assert_eq!(user.telephone, Some("8888".to_string()));
        assert_eq!(user.work_station, Some("工位A-101".to_string()));
        assert_eq!(
            user.department_ids,
            Some(vec!["dept_001".to_string(), "dept_002".to_string()])
        );
        assert_eq!(user.leader_user_id, Some(vec!["leader_001".to_string()]));
        assert_eq!(user.status, Some(UserStatus::Active));
        assert_eq!(user.is_hrm, Some(false));
        assert_eq!(user.create_time, Some("2023-01-01T00:00:00Z".to_string()));
        assert_eq!(user.update_time, Some("2023-12-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_user_gender_enum() {
        let male = UserGender::Male;
        let female = UserGender::Female;
        let unknown = UserGender::Unknown;

        assert_eq!(male, UserGender::Male);
        assert_eq!(female, UserGender::Female);
        assert_eq!(unknown, UserGender::Unknown);
        assert_ne!(male, female);
        assert_ne!(female, unknown);

        // Test default
        let default_gender = UserGender::default();
        assert_eq!(default_gender, UserGender::Unknown);
    }

    #[test]
    fn test_user_status_enum() {
        let active = UserStatus::Active;
        let inactive = UserStatus::Inactive;
        let disabled = UserStatus::Disabled;
        let deleted = UserStatus::Deleted;

        assert_eq!(active, UserStatus::Active);
        assert_eq!(inactive, UserStatus::Inactive);
        assert_eq!(disabled, UserStatus::Disabled);
        assert_eq!(deleted, UserStatus::Deleted);
        assert_ne!(active, inactive);
        assert_ne!(inactive, disabled);

        // Test default
        let default_status = UserStatus::default();
        assert_eq!(default_status, UserStatus::Inactive);
    }

    #[test]
    fn test_create_user_request() {
        let user = User {
            name: Some("李四".to_string()),
            email: Some("lisi@example.com".to_string()),
            mobile: Some("+8613900139000".to_string()),
            department_ids: Some(vec!["dept_001".to_string()]),
            position: Some("产品经理".to_string()),
            ..Default::default()
        };

        let request = CreateUserRequest {
            user: user.clone(),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(request.user.name, Some("李四".to_string()));
        assert_eq!(request.user.email, Some("lisi@example.com".to_string()));
        assert_eq!(request.user.mobile, Some("+8613900139000".to_string()));
        assert_eq!(
            request.user.department_ids,
            Some(vec!["dept_001".to_string()])
        );
        assert_eq!(request.user.position, Some("产品经理".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_get_user_request_default() {
        let request = GetUserRequest::default();
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_get_user_request_with_type() {
        let request = GetUserRequest {
            user_id_type: Some("union_id".to_string()),
        };

        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_batch_get_user_request() {
        let request = BatchGetUserRequest {
            user_ids: vec!["user_001".to_string(), "user_002".to_string()],
            user_id_type: Some("open_id".to_string()),
        };

        assert_eq!(request.user_ids.len(), 2);
        assert_eq!(request.user_ids[0], "user_001");
        assert_eq!(request.user_ids[1], "user_002");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_search_user_request_default() {
        let request = SearchUserRequest::default();
        assert_eq!(request.query, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_search_user_request_with_query() {
        let request = SearchUserRequest {
            query: Some("张".to_string()),
            user_id_type: Some("user_id".to_string()),
            page_size: Some(20),
            page_token: Some("search_token".to_string()),
        };

        assert_eq!(request.query, Some("张".to_string()));
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("search_token".to_string()));
    }

    #[test]
    fn test_get_user_by_department_request_default() {
        let request = GetUserByDepartmentRequest::default();
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_department_with_data() {
        let department = UserDepartment {
            department_id: Some("dept_001".to_string()),
            name: Some("技术部".to_string()),
            department_path: Some("/技术部".to_string()),
        };

        assert_eq!(department.department_id, Some("dept_001".to_string()));
        assert_eq!(department.name, Some("技术部".to_string()));
        assert_eq!(department.department_path, Some("/技术部".to_string()));
    }

    #[test]
    fn test_user_order_with_data() {
        let order = UserOrder {
            department_id: Some("dept_001".to_string()),
            order: Some(100),
        };

        assert_eq!(order.department_id, Some("dept_001".to_string()));
        assert_eq!(order.order, Some(100));
    }

    #[test]
    fn test_user_custom_attribute_with_data() {
        let attribute = UserCustomAttribute {
            id: Some("attr_001".to_string()),
            key: Some("custom_field".to_string()),
            value: Some(serde_json::json!("custom_value")),
        };

        assert_eq!(attribute.id, Some("attr_001".to_string()));
        assert_eq!(attribute.key, Some("custom_field".to_string()));
        assert!(attribute.value.is_some());
    }

    #[test]
    fn test_user_role_with_data() {
        let role = UserRole {
            name: Some("管理员".to_string()),
            role_type: Some("admin".to_string()),
        };

        assert_eq!(role.name, Some("管理员".to_string()));
        assert_eq!(role.role_type, Some("admin".to_string()));
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateUserResponse::default();
        let patch_response = PatchUserResponse::default();
        let update_response = UpdateUserResponse::default();
        let get_response = GetUserResponse::default();
        let delete_response = DeleteUserResponse::default();
        let batch_get_response = BatchGetUserResponse::default();
        let search_response = SearchUserResponse::default();
        let by_dept_response = GetUserByDepartmentResponse::default();
        let restore_response = RestoreUserResponse::default();

        assert!(!format!("{:?}", create_response).is_empty());
        assert!(!format!("{:?}", patch_response).is_empty());
        assert!(!format!("{:?}", update_response).is_empty());
        assert!(!format!("{:?}", get_response).is_empty());
        assert!(!format!("{:?}", delete_response).is_empty());
        assert_eq!(batch_get_response.users.len(), 0);
        assert_eq!(search_response.users.len(), 0);
        assert_eq!(by_dept_response.users.len(), 0);
        assert!(!format!("{:?}", restore_response).is_empty());
    }

    #[test]
    fn test_response_with_data() {
        let mut create_response = CreateUserResponse::default();
        create_response.user = User {
            user_id: Some("user_new".to_string()),
            name: Some("新用户".to_string()),
            ..Default::default()
        };

        assert_eq!(create_response.user.user_id, Some("user_new".to_string()));
        assert_eq!(create_response.user.name, Some("新用户".to_string()));

        let mut batch_get_response = BatchGetUserResponse::default();
        batch_get_response.users = vec![
            User {
                user_id: Some("user_001".to_string()),
                name: Some("用户1".to_string()),
                ..Default::default()
            },
            User {
                user_id: Some("user_002".to_string()),
                name: Some("用户2".to_string()),
                ..Default::default()
            },
        ];
        batch_get_response.has_more = Some(true);
        batch_get_response.page_token = Some("next_page".to_string());

        assert_eq!(batch_get_response.users.len(), 2);
        assert_eq!(
            batch_get_response.users[0].user_id,
            Some("user_001".to_string())
        );
        assert_eq!(batch_get_response.has_more, Some(true));
        assert_eq!(batch_get_response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementations() {
        assert_eq!(CreateUserResponse::data_format(), ResponseFormat::Data);
        assert_eq!(PatchUserResponse::data_format(), ResponseFormat::Data);
        assert_eq!(UpdateUserResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetUserResponse::data_format(), ResponseFormat::Data);
        assert_eq!(DeleteUserResponse::data_format(), ResponseFormat::Data);
        assert_eq!(BatchGetUserResponse::data_format(), ResponseFormat::Data);
        assert_eq!(SearchUserResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            GetUserByDepartmentResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(RestoreUserResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_user_builder() {
        let builder = CreateUserBuilder::new()
            .name("王五")
            .email("wangwu@example.com")
            .mobile("+8613700137000")
            .position("设计师")
            .employee_no("EMP002")
            .gender(UserGender::Female)
            .user_id_type("open_id")
            .department_id_type("department_id");

        assert_eq!(builder.request.user.name, Some("王五".to_string()));
        assert_eq!(
            builder.request.user.email,
            Some("wangwu@example.com".to_string())
        );
        assert_eq!(
            builder.request.user.mobile,
            Some("+8613700137000".to_string())
        );
        assert_eq!(builder.request.user.position, Some("设计师".to_string()));
        assert_eq!(builder.request.user.employee_no, Some("EMP002".to_string()));
        assert_eq!(builder.request.user.gender, Some(UserGender::Female));
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            builder.request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_search_user_builder() {
        let builder = SearchUserBuilder::new()
            .query("技术")
            .user_id_type("user_id")
            .page_size(30)
            .page_token("search_page_token");

        assert_eq!(builder.request.query, Some("技术".to_string()));
        assert_eq!(builder.request.user_id_type, Some("user_id".to_string()));
        assert_eq!(builder.request.page_size, Some(30));
        assert_eq!(
            builder.request.page_token,
            Some("search_page_token".to_string())
        );
    }

    #[test]
    fn test_request_serialization() {
        let user = User {
            name: Some("测试用户".to_string()),
            email: Some("test@example.com".to_string()),
            mobile: Some("+8613600136000".to_string()),
            department_ids: Some(vec!["dept_test".to_string()]),
            ..Default::default()
        };

        let request = CreateUserRequest {
            user,
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateUserRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.user.name, deserialized.user.name);
        assert_eq!(request.user.email, deserialized.user.email);
        assert_eq!(request.user_id_type, deserialized.user_id_type);
        assert_eq!(request.department_id_type, deserialized.department_id_type);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = GetUserByDepartmentRequest {
            user_id_type: Some("union_id".to_string()),
            page_size: Some(25),
            page_token: Some("dept_page_token".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&"user_id_type=union_id".to_string()));
        assert!(query_params.contains(&"page_size=25".to_string()));
        assert!(query_params.contains(&"page_token=dept_page_token".to_string()));
    }

    #[test]
    fn test_user_complex_scenarios() {
        // Test user with all fields
        let comprehensive_user = User {
            user_id: Some("user_comprehensive".to_string()),
            union_id: Some("union_comprehensive".to_string()),
            open_id: Some("open_comprehensive".to_string()),
            name: Some("综合用户".to_string()),
            en_name: Some("Comprehensive User".to_string()),
            nickname: Some("小综".to_string()),
            email: Some("comprehensive@example.com".to_string()),
            mobile: Some("+8613800138000".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
            avatar_72x72: Some("https://example.com/avatar_72.jpg".to_string()),
            avatar_240x240: Some("https://example.com/avatar_240.jpg".to_string()),
            avatar_640x640: Some("https://example.com/avatar_640.jpg".to_string()),
            avatar_original: Some("https://example.com/avatar_original.jpg".to_string()),
            gender: Some(UserGender::Male),
            employee_no: Some("COMP001".to_string()),
            position: Some("综合开发工程师".to_string()),
            telephone: Some("88888888".to_string()),
            work_station: Some("综合工位".to_string()),
            department_ids: Some(vec![
                "dept_001".to_string(),
                "dept_002".to_string(),
                "dept_003".to_string(),
            ]),
            departments: Some(vec![
                UserDepartment {
                    department_id: Some("dept_001".to_string()),
                    name: Some("技术部".to_string()),
                    department_path: Some("/技术部".to_string()),
                },
                UserDepartment {
                    department_id: Some("dept_002".to_string()),
                    name: Some("产品部".to_string()),
                    department_path: Some("/产品部".to_string()),
                },
            ]),
            leader_user_id: Some(vec!["leader_001".to_string(), "leader_002".to_string()]),
            orders: Some(vec![
                UserOrder {
                    department_id: Some("dept_001".to_string()),
                    order: Some(1),
                },
                UserOrder {
                    department_id: Some("dept_002".to_string()),
                    order: Some(2),
                },
            ]),
            custom_attributes: Some(vec![UserCustomAttribute {
                id: Some("attr_001".to_string()),
                key: Some("skill".to_string()),
                value: Some(serde_json::json!("Rust")),
            }]),
            status: Some(UserStatus::Active),
            is_hrm: Some(false),
            roles: Some(vec![UserRole {
                name: Some("开发者".to_string()),
                role_type: Some("dev".to_string()),
            }]),
            user_extensions: Some(UserExtensions {
                extra: Some(serde_json::json!({"bio": "资深开发者"})),
            }),
            employee_type: Some(1),
            wallet_address: Some("0x1234567890abcdef".to_string()),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
        };

        assert_eq!(
            comprehensive_user.user_id,
            Some("user_comprehensive".to_string())
        );
        assert_eq!(
            comprehensive_user.union_id,
            Some("union_comprehensive".to_string())
        );
        assert_eq!(
            comprehensive_user.open_id,
            Some("open_comprehensive".to_string())
        );
        assert_eq!(comprehensive_user.name, Some("综合用户".to_string()));
        assert_eq!(
            comprehensive_user.en_name,
            Some("Comprehensive User".to_string())
        );
        assert_eq!(comprehensive_user.nickname, Some("小综".to_string()));
        assert_eq!(
            comprehensive_user.email,
            Some("comprehensive@example.com".to_string())
        );
        assert_eq!(
            comprehensive_user.mobile,
            Some("+8613800138000".to_string())
        );
        assert_eq!(
            comprehensive_user.avatar,
            Some("https://example.com/avatar.jpg".to_string())
        );
        assert_eq!(
            comprehensive_user.avatar_72x72,
            Some("https://example.com/avatar_72.jpg".to_string())
        );
        assert_eq!(
            comprehensive_user.avatar_240x240,
            Some("https://example.com/avatar_240.jpg".to_string())
        );
        assert_eq!(
            comprehensive_user.avatar_640x640,
            Some("https://example.com/avatar_640.jpg".to_string())
        );
        assert_eq!(
            comprehensive_user.avatar_original,
            Some("https://example.com/avatar_original.jpg".to_string())
        );
        assert_eq!(comprehensive_user.gender, Some(UserGender::Male));
        assert_eq!(comprehensive_user.employee_no, Some("COMP001".to_string()));
        assert_eq!(
            comprehensive_user.position,
            Some("综合开发工程师".to_string())
        );
        assert_eq!(comprehensive_user.telephone, Some("88888888".to_string()));
        assert_eq!(
            comprehensive_user.work_station,
            Some("综合工位".to_string())
        );
        assert_eq!(comprehensive_user.department_ids.as_ref().unwrap().len(), 3);
        assert_eq!(comprehensive_user.leader_user_id.as_ref().unwrap().len(), 2);
        assert_eq!(comprehensive_user.orders.as_ref().unwrap().len(), 2);
        assert_eq!(
            comprehensive_user.custom_attributes.as_ref().unwrap().len(),
            1
        );
        assert_eq!(comprehensive_user.status, Some(UserStatus::Active));
        assert_eq!(comprehensive_user.is_hrm, Some(false));
        assert_eq!(comprehensive_user.roles.as_ref().unwrap().len(), 1);
        assert!(comprehensive_user.user_extensions.is_some());
        assert_eq!(comprehensive_user.employee_type, Some(1));
        assert_eq!(
            comprehensive_user.wallet_address,
            Some("0x1234567890abcdef".to_string())
        );
        assert_eq!(
            comprehensive_user.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_user.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
    }

    #[test]
    fn test_user_status_variations() {
        // Test different status values
        let active_user = User {
            name: Some("活跃用户".to_string()),
            status: Some(UserStatus::Active),
            ..Default::default()
        };

        let inactive_user = User {
            name: Some("未激活用户".to_string()),
            status: Some(UserStatus::Inactive),
            ..Default::default()
        };

        let disabled_user = User {
            name: Some("禁用用户".to_string()),
            status: Some(UserStatus::Disabled),
            ..Default::default()
        };

        let deleted_user = User {
            name: Some("已删除用户".to_string()),
            status: Some(UserStatus::Deleted),
            ..Default::default()
        };

        assert_eq!(active_user.status, Some(UserStatus::Active));
        assert_eq!(inactive_user.status, Some(UserStatus::Inactive));
        assert_eq!(disabled_user.status, Some(UserStatus::Disabled));
        assert_eq!(deleted_user.status, Some(UserStatus::Deleted));
        assert_ne!(active_user.status, inactive_user.status);
    }

    #[test]
    fn test_user_gender_variations() {
        // Test different gender values
        let male_user = User {
            name: Some("男性用户".to_string()),
            gender: Some(UserGender::Male),
            ..Default::default()
        };

        let female_user = User {
            name: Some("女性用户".to_string()),
            gender: Some(UserGender::Female),
            ..Default::default()
        };

        let unknown_gender_user = User {
            name: Some("未知性别用户".to_string()),
            gender: Some(UserGender::Unknown),
            ..Default::default()
        };

        let no_gender_user = User {
            name: Some("无性别用户".to_string()),
            gender: None,
            ..Default::default()
        };

        assert_eq!(male_user.gender, Some(UserGender::Male));
        assert_eq!(female_user.gender, Some(UserGender::Female));
        assert_eq!(unknown_gender_user.gender, Some(UserGender::Unknown));
        assert!(no_gender_user.gender.is_none());
    }

    #[test]
    fn test_department_ids_variations() {
        // Test with different department configurations
        let single_dept_user = User {
            name: Some("单部门用户".to_string()),
            department_ids: Some(vec!["dept_001".to_string()]),
            ..Default::default()
        };

        let multiple_depts_user = User {
            name: Some("多部门用户".to_string()),
            department_ids: Some(vec![
                "dept_002".to_string(),
                "dept_003".to_string(),
                "dept_004".to_string(),
            ]),
            ..Default::default()
        };

        let no_dept_user = User {
            name: Some("无部门用户".to_string()),
            department_ids: None,
            ..Default::default()
        };

        assert_eq!(single_dept_user.department_ids.as_ref().unwrap().len(), 1);
        assert_eq!(
            multiple_depts_user.department_ids.as_ref().unwrap().len(),
            3
        );
        assert!(no_dept_user.department_ids.is_none());
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_USERS,
            "/open-apis/contact/v3/users"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_USER_GET,
            "/open-apis/contact/v3/users/{user_id}"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_USERS_BATCH,
            "/open-apis/contact/v3/users/batch"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_USERS_SEARCH,
            "/open-apis/contact/v3/users/search"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_USERS_FIND_BY_DEPARTMENT,
            "/open-apis/contact/v3/users/find_by_department"
        );
    }
}
