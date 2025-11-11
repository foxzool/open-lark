#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Persons v1 - 人员管理API
//!
//! 提供完整的人员生命周期管理功能，包括：
//! - 人员信息的增删改查操作
//! - 人员状态管理和批量处理
//! - 人员搜索和高级筛选功能
//! - 人员头像管理和上传
//! - 部门人员关系维护
//! - 人员基础信息查询
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::feishu_people::core::v1::persons::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 获取人员详情
//!     let response = client.feishu_people.core.v1.persons
//!         .get_person_builder("user_001")
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.persons)
//!         .await?;
//!
//!     println!("人员姓名: {}", response.data.name);
//!
//!     // 批量获取人员信息
//!     let response = client.feishu_people.core.v1.persons
//!         .batch_get_persons_builder()
//!         .user_ids(vec!["user_001".to_string(), "user_002".to_string()])
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.persons)
//!         .await?;
//!
//!     println!("获取到 {} 个人员", response.data.items.len());
//!
//!     // 搜索人员
//!     let response = client.feishu_people.core.v1.persons
//!         .search_persons_builder()
//!         .query("张三")
//!         .page_size(20)
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.persons)
//!         .await?;
//!
//!     println!("搜索到 {} 个人员", response.data.items.len());
//!
//!     Ok(())
//! }
//! ```

use crate::{
    api_resp::{ApiResponseTrait, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};
use open_lark_core::core::{
    api_req::ApiRequest, error::LarkAPIError as CoreLarkAPIError, SDKResult as CoreSDKResult,
};
use serde::{Deserialize, Serialize};

/// 人员管理服务
///
/// 提供完整的人员生命周期管理功能，包括人员的创建、更新、删除、查询等操作。
/// 支持企业级的人员管理需求，包括批量操作、高级搜索和统计分析功能。
///
/// # 核心功能
///
/// - **人员CRUD操作**: 创建、查询、更新、删除人员信息
/// - **批量处理**: 支持批量获取和操作人员数据
/// - **高级搜索**: 基于多种条件的人员搜索和筛选
/// - **状态管理**: 人员状态的更新和维护
/// - **头像管理**: 人员头像的获取和上传
/// - **基础信息**: 人员基础信息的快速查询
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::feishu_people::core::v1::persons::PersonsService;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = PersonsService::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct PersonsService {
    pub config: Config,
}

impl PersonsService {
    /// 创建人员服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::PersonsService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = PersonsService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取人员详情
    ///
    /// 根据用户ID获取人员的详细信息，包括基本信息、联系方式、职位等。
    /// 支持使用不同的用户ID类型进行查询，返回完整的人员配置信息。
    ///
    /// # API文档
    ///
    /// 根据用户ID获取人员的详细信息。
    /// 返回人员的基本信息、联系方式、职位、部门等完整资料。
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 获取人员的请求参数，包含ID类型配置
    ///
    /// # 返回值
    ///
    /// 返回人员的详细信息，包含基本信息、联系方式、职位等完整数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let request = GetPersonRequest {
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .get("user_001", &request).await?;
    /// println!("人员姓名: {}", response.data.name);
    /// ```
    pub async fn get(
        &self,
        user_id: &str,
        request: &GetPersonRequest,
    ) -> SDKResult<BaseResponse<GetPersonResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/feishu_people/core/v1/persons/{}", user_id),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 批量获取人员信息
    ///
    /// 根据用户ID列表批量获取多个人员的详细信息。
    /// 适用于需要同时查询多个人员信息的场景，提高查询效率。
    ///
    /// # API文档
    ///
    /// 根据用户ID列表批量获取人员信息。
    /// 最多支持50个用户ID的批量查询，返回人员的完整信息。
    ///
    /// # 参数
    ///
    /// * `request` - 批量获取人员的请求参数，包含用户ID列表和配置
    ///
    /// # 返回值
    ///
    /// 返回批量人员的详细信息列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let request = BatchGetPersonsRequest {
    ///     user_ids: vec!["user_001".to_string(), "user_002".to_string()],
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .batch_get(&request).await?;
    /// println!("获取到 {} 个人员", response.data.items.len());
    /// ```
    pub async fn batch_get(
        &self,
        request: &BatchGetPersonsRequest,
    ) -> SDKResult<BaseResponse<BatchGetPersonsResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/persons/batch_get".to_string(),
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 根据部门获取人员列表
    ///
    /// 获取指定部门下的人员列表，支持分页查询。
    /// 可以用于了解部门的人员配置和分布情况。
    ///
    /// # API文档
    ///
    /// 根据部门ID获取该部门下的人员列表。
    /// 支持分页查询，返回部门内所有人员的基本信息。
    ///
    /// # 参数
    ///
    /// * `request` - 获取部门人员列表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回部门人员列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let request = GetPersonsByDepartmentRequest {
    ///     department_id: "dept_001".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .get_by_department(&request).await?;
    /// println!("部门人员数量: {}", response.data.items.len());
    /// ```
    pub async fn get_by_department(
        &self,
        request: &GetPersonsByDepartmentRequest,
    ) -> SDKResult<BaseResponse<GetPersonsByDepartmentResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("department_id", request.department_id.clone());
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token", page_token.clone());
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/feishu_people/core/v1/persons/list_by_department".to_string(),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 搜索人员
    ///
    /// 根据关键词搜索人员，支持按姓名、邮箱、手机号、职位等字段进行搜索。
    /// 返回匹配的人员列表，支持分页查询和相关性排序。
    ///
    /// # API文档
    ///
    /// 根据关键词搜索人员信息。
    /// 支持按人员姓名、邮箱、手机号、职位等多种字段进行模糊搜索。
    ///
    /// # 参数
    ///
    /// * `request` - 搜索人员的请求参数，包含搜索关键词和过滤条件
    ///
    /// # 返回值
    ///
    /// 返回搜索结果和分页信息，包含匹配度评分
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let request = SearchPersonsRequest {
    ///     query: "张三".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .search(&request).await?;
    /// println!("搜索到 {} 个人员", response.data.items.len());
    /// ```
    pub async fn search(
        &self,
        request: &SearchPersonsRequest,
    ) -> SDKResult<BaseResponse<SearchPersonsResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/persons/search".to_string(),
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 更新人员信息
    ///
    /// 更新人员的信息，支持修改人员的基本属性、联系方式、职位等。
    /// 只更新传入的字段，未传入的字段保持不变。
    ///
    /// # API文档
    ///
    /// 修改人员的部分信息，只更新提供的字段。
    /// 支持修改人员的姓名、联系方式、职位、部门等信息。
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 修改人员的请求参数
    ///
    /// # 返回值
    ///
    /// 返回修改后的人员信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let update_data = PersonUpdateData {
    ///     name: Some("张三(更新)".to_string()),
    ///     email: Some("zhangsan_new@example.com".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = UpdatePersonRequest {
    ///     person_data: update_data,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .update("user_001", &request).await?;
    /// println!("人员信息更新成功");
    /// ```
    pub async fn update(
        &self,
        user_id: &str,
        request: &UpdatePersonRequest,
    ) -> SDKResult<BaseResponse<UpdatePersonResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: format!("/open-apis/feishu_people/core/v1/persons/{}", user_id),
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 更新人员状态
    ///
    /// 更新人员的状态信息，如在职、离职、停职等。
    /// 用于人员状态的生命周期管理。
    ///
    /// # API文档
    ///
    /// 更新人员的状态信息。
    /// 支持设置人员的在职状态、离职状态等。
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 更新人员状态的请求参数
    ///
    /// # 返回值
    ///
    /// 返回更新操作的结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let request = UpdatePersonStatusRequest {
    ///     status: "active".to_string(),
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .update_status("user_001", &request).await?;
    /// println!("人员状态更新成功");
    /// ```
    pub async fn update_status(
        &self,
        user_id: &str,
        request: &UpdatePersonStatusRequest,
    ) -> SDKResult<BaseResponse<UpdatePersonStatusResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: format!(
                "/open-apis/feishu_people/core/v1/persons/{}/status",
                user_id
            ),
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取用户头像
    ///
    /// 获取指定用户的头像信息，支持多种尺寸的头像图片。
    /// 返回不同规格的头像URL，适用于不同的显示场景。
    ///
    /// # API文档
    ///
    /// 获取用户头像信息，支持多种尺寸。
    /// 返回72x72、240x240、360x360、640x640、1024x1024等不同尺寸的头像URL。
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 获取用户头像的请求参数
    ///
    /// # 返回值
    ///
    /// 返回用户头像信息，包含不同尺寸的头像URL
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let request = GetPersonAvatarRequest {
    ///     size: Some("240".to_string()),
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .get_avatar("user_001", &request).await?;
    /// println!("头像URL: {:?}", response.data.avatar_240);
    /// ```
    pub async fn get_avatar(
        &self,
        user_id: &str,
        request: &GetPersonAvatarRequest,
    ) -> SDKResult<BaseResponse<GetPersonAvatarResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(size) = &request.size {
            query_params.insert("size", size.clone());
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/feishu_people/core/v1/persons/{}/avatar",
                user_id
            ),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 上传用户头像
    ///
    /// 为指定用户上传头像图片，支持多种图片格式。
    /// 上传成功后会自动生成不同尺寸的头像版本。
    ///
    /// # API文档
    ///
    /// 上传用户头像图片，支持JPG、PNG等格式。
    /// 系统会自动生成72x72、240x240、360x360、640x640、1024x1024等不同尺寸的版本。
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 上传用户头像的请求参数
    ///
    /// # 返回值
    ///
    /// 返回上传结果，包含不同尺寸的头像URL
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let image_data = std::fs::read("avatar.jpg").unwrap();
    /// let request = UploadPersonAvatarRequest {
    ///     image_data,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .upload_avatar("user_001", &request).await?;
    /// println!("头像上传成功");
    /// ```
    pub async fn upload_avatar(
        &self,
        user_id: &str,
        request: &UploadPersonAvatarRequest,
    ) -> SDKResult<BaseResponse<UploadPersonAvatarResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/feishu_people/core/v1/persons/{}/avatar",
                user_id
            ),
            body: request.image_data.clone(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取人员基础信息
    ///
    /// 获取人员的基础信息，包含姓名、联系方式、头像等核心字段。
    /// 适用于需要快速显示人员基本信息的场景。
    ///
    /// # API文档
    ///
    /// 获取人员的基础信息，包含核心字段。
    /// 返回人员姓名、邮箱、手机号、头像等基本信息。
    ///
    /// # 参数
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 获取人员基础信息的请求参数
    ///
    /// # 返回值
    ///
    /// 返回人员的基础信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let request = GetPersonBasicInfoRequest {
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.persons
    ///     .get_basic_info("user_001", &request).await?;
    /// println!("人员姓名: {}", response.data.name);
    /// ```
    pub async fn get_basic_info(
        &self,
        user_id: &str,
        request: &GetPersonBasicInfoRequest,
    ) -> SDKResult<BaseResponse<GetPersonBasicInfoResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/feishu_people/core/v1/persons/{}/basic_info",
                user_id
            ),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    // ==================== Builder模式实现 ====================

    /// 获取人员详情构建器
    ///
    /// 提供流式API来构建获取人员详情的请求参数。
    /// 支持链式调用，方便配置查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let builder = client.feishu_people.core.v1.persons
    ///     .get_person_builder("user_001")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn get_person_builder(&self, user_id: &str) -> GetPersonBuilder {
        GetPersonBuilder::new(user_id)
    }

    /// 批量获取人员构建器
    ///
    /// 提供流式API来构建批量获取人员的请求参数。
    /// 支持链式调用，方便配置批量查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let builder = client.feishu_people.core.v1.persons
    ///     .batch_get_persons_builder()
    ///     .user_ids(vec!["user_001".to_string(), "user_002".to_string()])
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn batch_get_persons_builder(&self) -> BatchGetPersonsBuilder {
        BatchGetPersonsBuilder::new()
    }

    /// 根据部门获取人员列表构建器
    ///
    /// 提供流式API来构建获取部门人员列表的请求参数。
    /// 支持链式调用，方便配置分页和过滤参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let builder = client.feishu_people.core.v1.persons
    ///     .get_persons_by_department_builder("dept_001")
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn get_persons_by_department_builder(
        &self,
        department_id: &str,
    ) -> GetPersonsByDepartmentBuilder {
        GetPersonsByDepartmentBuilder::new(department_id)
    }

    /// 搜索人员构建器
    ///
    /// 提供流式API来构建搜索人员的请求参数。
    /// 支持链式调用，方便配置搜索条件和分页参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let builder = client.feishu_people.core.v1.persons
    ///     .search_persons_builder()
    ///     .query("张三".to_string())
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn search_persons_builder(&self) -> SearchPersonsBuilder {
        SearchPersonsBuilder::new()
    }

    /// 更新人员信息构建器
    ///
    /// 提供流式API来构建更新人员信息的请求参数。
    /// 支持链式调用，方便构建人员更新请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let update_data = PersonUpdateData {
    ///     name: Some("张三(更新)".to_string()),
    ///     email: Some("zhangsan_new@example.com".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.feishu_people.core.v1.persons
    ///     .update_person_builder("user_001")
    ///     .person_data(update_data)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn update_person_builder(&self, user_id: &str) -> UpdatePersonBuilder {
        UpdatePersonBuilder::new(user_id)
    }

    /// 更新人员状态构建器
    ///
    /// 提供流式API来构建更新人员状态的请求参数。
    /// 支持链式调用，方便配置状态更新参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let builder = client.feishu_people.core.v1.persons
    ///     .update_person_status_builder("user_001")
    ///     .status("active")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn update_person_status_builder(&self, user_id: &str) -> UpdatePersonStatusBuilder {
        UpdatePersonStatusBuilder::new(user_id)
    }

    /// 获取用户头像构建器
    ///
    /// 提供流式API来构建获取用户头像的请求参数。
    /// 支持链式调用，方便配置头像尺寸和查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let builder = client.feishu_people.core.v1.persons
    ///     .get_person_avatar_builder("user_001")
    ///     .size("240")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn get_person_avatar_builder(&self, user_id: &str) -> GetPersonAvatarBuilder {
        GetPersonAvatarBuilder::new(user_id)
    }

    /// 上传用户头像构建器
    ///
    /// 提供流式API来构建上传用户头像的请求参数。
    /// 支持链式调用，方便配置图片数据和上传参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let image_data = std::fs::read("avatar.jpg").unwrap();
    /// let builder = client.feishu_people.core.v1.persons
    ///     .upload_person_avatar_builder("user_001")
    ///     .image_data(image_data)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn upload_person_avatar_builder(&self, user_id: &str) -> UploadPersonAvatarBuilder {
        UploadPersonAvatarBuilder::new(user_id)
    }

    /// 获取人员基础信息构建器
    ///
    /// 提供流式API来构建获取人员基础信息的请求参数。
    /// 支持链式调用，方便配置查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::persons::*;
    ///
    /// let builder = client.feishu_people.core.v1.persons
    ///     .get_person_basic_info_builder("user_001")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.persons).await?;
    /// ```
    pub fn get_person_basic_info_builder(&self, user_id: &str) -> GetPersonBasicInfoBuilder {
        GetPersonBasicInfoBuilder::new(user_id)
    }
}

// ==================== Builder结构体实现 ====================

/// 获取人员详情构建器
#[derive(Debug, Clone)]
pub struct GetPersonBuilder {
    user_id: String,
    request: GetPersonRequest,
}

impl GetPersonBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: GetPersonRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, GetPersonRequest) {
        (self.user_id, self.request)
    }
}

impl Default for GetPersonBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

fn map_sdk_error(err: LarkAPIError) -> CoreLarkAPIError {
    match err {
        LarkAPIError::IOErr(msg) => CoreLarkAPIError::IOErr(msg),
        LarkAPIError::IllegalParamError(msg) => CoreLarkAPIError::IllegalParamError(msg),
        LarkAPIError::DeserializeError(msg) => CoreLarkAPIError::DeserializeError(msg),
        LarkAPIError::RequestError(msg) => CoreLarkAPIError::RequestError(msg),
        LarkAPIError::UrlParseError(msg) => CoreLarkAPIError::UrlParseError(msg),
        LarkAPIError::ApiError {
            code,
            message,
            request_id,
        } => CoreLarkAPIError::ApiError {
            code,
            message,
            request_id,
        },
        LarkAPIError::MissingAccessToken => CoreLarkAPIError::MissingAccessToken,
        LarkAPIError::BadRequest(msg) => CoreLarkAPIError::BadRequest(msg),
        LarkAPIError::DataError(msg) => CoreLarkAPIError::DataError(msg),
        LarkAPIError::APIError { code, msg, error } => {
            CoreLarkAPIError::APIError { code, msg, error }
        }
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        (String, GetPersonRequest),
        BaseResponse<GetPersonResponse>,
    > for GetPersonBuilder
{
    fn build(self) -> (String, GetPersonRequest) {
        (self.user_id, self.request)
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<GetPersonResponse>> {
        service
            .get_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<GetPersonResponse>> {
        service
            .get_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }
}

/// 批量获取人员构建器
#[derive(Debug, Clone)]
pub struct BatchGetPersonsBuilder {
    request: BatchGetPersonsRequest,
}

impl BatchGetPersonsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: BatchGetPersonsRequest::default(),
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

    /// 构建最终的请求对象
    pub fn build(self) -> BatchGetPersonsRequest {
        self.request
    }
}

impl Default for BatchGetPersonsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait - 标准实现
// 使用自定义实现，因为batch_get方法不接受Option参数
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        BatchGetPersonsRequest,
        BaseResponse<BatchGetPersonsResponse>,
    > for BatchGetPersonsBuilder
{
    fn build(self) -> BatchGetPersonsRequest {
        self.request
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<BatchGetPersonsResponse>> {
        service
            .batch_get(&self.build())
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<BatchGetPersonsResponse>> {
        service
            .batch_get(&self.build())
            .await
            .map_err(map_sdk_error)
    }
}

/// 根据部门获取人员列表构建器
#[derive(Debug, Clone)]
pub struct GetPersonsByDepartmentBuilder {
    request: GetPersonsByDepartmentRequest,
}

impl GetPersonsByDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new(department_id: &str) -> Self {
        Self {
            request: GetPersonsByDepartmentRequest {
                department_id: department_id.to_string(),
                page_size: None,
                page_token: None,
                user_id_type: None,
            },
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

    /// 构建最终的请求对象
    pub fn build(self) -> GetPersonsByDepartmentRequest {
        self.request
    }
}

impl Default for GetPersonsByDepartmentBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        GetPersonsByDepartmentRequest,
        BaseResponse<GetPersonsByDepartmentResponse>,
    > for GetPersonsByDepartmentBuilder
{
    fn build(self) -> GetPersonsByDepartmentRequest {
        self.request
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<GetPersonsByDepartmentResponse>> {
        service
            .get_by_department(&self.build())
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<GetPersonsByDepartmentResponse>> {
        service
            .get_by_department(&self.build())
            .await
            .map_err(map_sdk_error)
    }
}

/// 搜索人员构建器
#[derive(Debug, Clone)]
pub struct SearchPersonsBuilder {
    request: SearchPersonsRequest,
}

impl SearchPersonsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchPersonsRequest::default(),
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

    /// 构建最终的请求对象
    pub fn build(self) -> SearchPersonsRequest {
        self.request
    }
}

impl Default for SearchPersonsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        SearchPersonsRequest,
        BaseResponse<SearchPersonsResponse>,
    > for SearchPersonsBuilder
{
    fn build(self) -> SearchPersonsRequest {
        self.request
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<SearchPersonsResponse>> {
        service.search(&self.build()).await.map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<SearchPersonsResponse>> {
        service.search(&self.build()).await.map_err(map_sdk_error)
    }
}

/// 更新人员信息构建器
#[derive(Debug, Clone)]
pub struct UpdatePersonBuilder {
    user_id: String,
    request: UpdatePersonRequest,
}

impl UpdatePersonBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: UpdatePersonRequest::default(),
        }
    }

    /// 设置人员数据
    pub fn person_data(mut self, person_data: PersonUpdateData) -> Self {
        self.request.person_data = person_data;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, UpdatePersonRequest) {
        (self.user_id, self.request)
    }
}

impl Default for UpdatePersonBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        (String, UpdatePersonRequest),
        BaseResponse<UpdatePersonResponse>,
    > for UpdatePersonBuilder
{
    fn build(self) -> (String, UpdatePersonRequest) {
        (self.user_id, self.request)
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<UpdatePersonResponse>> {
        service
            .update_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<UpdatePersonResponse>> {
        service
            .update_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }
}

/// 更新人员状态构建器
#[derive(Debug, Clone)]
pub struct UpdatePersonStatusBuilder {
    user_id: String,
    request: UpdatePersonStatusRequest,
}

impl UpdatePersonStatusBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: UpdatePersonStatusRequest::default(),
        }
    }

    /// 设置人员状态
    pub fn status(mut self, status: &str) -> Self {
        self.request.status = status.to_string();
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, UpdatePersonStatusRequest) {
        (self.user_id, self.request)
    }
}

impl Default for UpdatePersonStatusBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        (String, UpdatePersonStatusRequest),
        BaseResponse<UpdatePersonStatusResponse>,
    > for UpdatePersonStatusBuilder
{
    fn build(self) -> (String, UpdatePersonStatusRequest) {
        (self.user_id, self.request)
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<UpdatePersonStatusResponse>> {
        service
            .update_status_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<UpdatePersonStatusResponse>> {
        service
            .update_status_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }
}

/// 获取用户头像构建器
#[derive(Debug, Clone)]
pub struct GetPersonAvatarBuilder {
    user_id: String,
    request: GetPersonAvatarRequest,
}

impl GetPersonAvatarBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: GetPersonAvatarRequest::default(),
        }
    }

    /// 设置头像尺寸
    pub fn size(mut self, size: &str) -> Self {
        self.request.size = Some(size.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, GetPersonAvatarRequest) {
        (self.user_id, self.request)
    }
}

impl Default for GetPersonAvatarBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        (String, GetPersonAvatarRequest),
        BaseResponse<GetPersonAvatarResponse>,
    > for GetPersonAvatarBuilder
{
    fn build(self) -> (String, GetPersonAvatarRequest) {
        (self.user_id, self.request)
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<GetPersonAvatarResponse>> {
        service
            .get_avatar_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<GetPersonAvatarResponse>> {
        service
            .get_avatar_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }
}

/// 上传用户头像构建器
#[derive(Debug, Clone)]
pub struct UploadPersonAvatarBuilder {
    user_id: String,
    request: UploadPersonAvatarRequest,
}

impl UploadPersonAvatarBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: UploadPersonAvatarRequest::default(),
        }
    }

    /// 设置图片数据
    pub fn image_data(mut self, image_data: Vec<u8>) -> Self {
        self.request.image_data = image_data;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, UploadPersonAvatarRequest) {
        (self.user_id, self.request)
    }
}

impl Default for UploadPersonAvatarBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        (String, UploadPersonAvatarRequest),
        BaseResponse<UploadPersonAvatarResponse>,
    > for UploadPersonAvatarBuilder
{
    fn build(self) -> (String, UploadPersonAvatarRequest) {
        (self.user_id, self.request)
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<UploadPersonAvatarResponse>> {
        service
            .upload_avatar_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<UploadPersonAvatarResponse>> {
        service
            .upload_avatar_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }
}

/// 获取人员基础信息构建器
#[derive(Debug, Clone)]
pub struct GetPersonBasicInfoBuilder {
    user_id: String,
    request: GetPersonBasicInfoRequest,
}

impl GetPersonBasicInfoBuilder {
    /// 创建新的Builder实例
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            request: GetPersonBasicInfoRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, GetPersonBasicInfoRequest) {
        (self.user_id, self.request)
    }
}

impl Default for GetPersonBasicInfoBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        PersonsService,
        (String, GetPersonBasicInfoRequest),
        BaseResponse<GetPersonBasicInfoResponse>,
    > for GetPersonBasicInfoBuilder
{
    fn build(self) -> (String, GetPersonBasicInfoRequest) {
        (self.user_id, self.request)
    }

    async fn execute(
        self,
        service: &PersonsService,
    ) -> CoreSDKResult<BaseResponse<GetPersonBasicInfoResponse>> {
        service
            .get_basic_info_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &PersonsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<GetPersonBasicInfoResponse>> {
        service
            .get_basic_info_with_tuple(self.build())
            .await
            .map_err(map_sdk_error)
    }
}

// 为PersonsService实现辅助方法，处理Builder的参数
impl PersonsService {
    async fn get_with_tuple(
        &self,
        params: (String, GetPersonRequest),
    ) -> SDKResult<BaseResponse<GetPersonResponse>> {
        self.get(&params.0, &params.1).await
    }

    async fn update_with_tuple(
        &self,
        params: (String, UpdatePersonRequest),
    ) -> SDKResult<BaseResponse<UpdatePersonResponse>> {
        self.update(&params.0, &params.1).await
    }

    async fn update_status_with_tuple(
        &self,
        params: (String, UpdatePersonStatusRequest),
    ) -> SDKResult<BaseResponse<UpdatePersonStatusResponse>> {
        self.update_status(&params.0, &params.1).await
    }

    async fn get_avatar_with_tuple(
        &self,
        params: (String, GetPersonAvatarRequest),
    ) -> SDKResult<BaseResponse<GetPersonAvatarResponse>> {
        self.get_avatar(&params.0, &params.1).await
    }

    async fn upload_avatar_with_tuple(
        &self,
        params: (String, UploadPersonAvatarRequest),
    ) -> SDKResult<BaseResponse<UploadPersonAvatarResponse>> {
        self.upload_avatar(&params.0, &params.1).await
    }

    async fn get_basic_info_with_tuple(
        &self,
        params: (String, GetPersonBasicInfoRequest),
    ) -> SDKResult<BaseResponse<GetPersonBasicInfoResponse>> {
        self.get_basic_info(&params.0, &params.1).await
    }
}

// ==================== 数据模型 ====================

/// 获取人员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for GetPersonRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 获取人员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonResponse {
    /// 人员信息
    pub person: Person,
}

impl ApiResponseTrait for GetPersonResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

/// 批量获取人员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetPersonsRequest {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for BatchGetPersonsRequest {
    fn default() -> Self {
        Self {
            user_ids: vec![],
            user_id_type: None,
        }
    }
}

/// 批量获取人员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetPersonsResponse {
    /// 人员列表
    pub items: Vec<Person>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for BatchGetPersonsResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

/// 根据部门获取人员列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonsByDepartmentRequest {
    /// 部门ID
    pub department_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 根据部门获取人员列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonsByDepartmentResponse {
    /// 人员列表
    pub items: Vec<Person>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for GetPersonsByDepartmentResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

/// 搜索人员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPersonsRequest {
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
}

impl Default for SearchPersonsRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }
}

/// 搜索人员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPersonsResponse {
    /// 搜索结果项
    pub items: Vec<PersonSearchResult>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 总数
    pub total: Option<i32>,
}

/// 更新人员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonRequest {
    /// 人员数据
    pub person_data: PersonUpdateData,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for UpdatePersonRequest {
    fn default() -> Self {
        Self {
            person_data: PersonUpdateData::default(),
            user_id_type: None,
        }
    }
}

/// 更新人员响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonResponse {
    /// 人员信息
    pub person: Person,
}

/// 更新人员状态请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonStatusRequest {
    /// 人员状态
    pub status: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for UpdatePersonStatusRequest {
    fn default() -> Self {
        Self {
            status: String::new(),
            user_id_type: None,
        }
    }
}

/// 更新人员状态响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePersonStatusResponse {}

/// 获取用户头像请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonAvatarRequest {
    /// 头像尺寸
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for GetPersonAvatarRequest {
    fn default() -> Self {
        Self {
            size: None,
            user_id_type: None,
        }
    }
}

/// 获取用户头像响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonAvatarResponse {
    /// 头像数据
    pub avatar: PersonAvatar,
}

/// 上传用户头像请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPersonAvatarRequest {
    /// 图片数据
    pub image_data: Vec<u8>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for UploadPersonAvatarRequest {
    fn default() -> Self {
        Self {
            image_data: vec![],
            user_id_type: None,
        }
    }
}

/// 上传用户头像响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPersonAvatarResponse {
    /// 头像数据
    pub avatar: PersonAvatar,
}

/// 获取人员基础信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonBasicInfoRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for GetPersonBasicInfoRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 获取人员基础信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonBasicInfoResponse {
    /// 人员基础信息
    pub person: PersonBasicInfo,
}

/// 人员搜索结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonSearchResult {
    /// 用户ID
    pub user_id: String,
    /// 姓名
    pub name: String,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 部门名称
    pub department_names: Option<Vec<String>>,
    /// 职位
    pub position: Option<String>,
    /// 匹配分数
    pub match_score: f64,
}

/// 人员基础信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonBasicInfo {
    /// 用户ID
    pub user_id: String,
    /// 姓名
    pub name: String,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 员工类型
    pub employee_type: Option<String>,
    /// 入职时间
    pub join_time: Option<String>,
    /// 人员状态
    pub status: Option<String>,
}

/// 人员头像信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonAvatar {
    /// 72x72头像
    pub avatar_72: Option<String>,
    /// 240x240头像
    pub avatar_240: Option<String>,
    /// 360x360头像
    pub avatar_360: Option<String>,
    /// 640x640头像
    pub avatar_640: Option<String>,
    /// 1024x1024头像
    pub avatar_1024: Option<String>,
}

/// 人员信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Person {
    /// 用户ID
    pub user_id: String,
    /// 姓名
    pub name: String,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 员工类型
    pub employee_type: Option<String>,
    /// 入职时间
    pub join_time: Option<String>,
    /// 人员状态
    pub status: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 职位
    pub position: Option<String>,
    /// 工号
    pub employee_number: Option<String>,
    /// 工作地点
    pub work_station: Option<String>,
    /// 直属上级
    pub leader_user_id: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

/// 人员更新数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonUpdateData {
    /// 姓名
    pub name: Option<String>,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 员工类型
    pub employee_type: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 职位
    pub position: Option<String>,
    /// 工号
    pub employee_number: Option<String>,
    /// 工作地点
    pub work_station: Option<String>,
    /// 直属上级
    pub leader_user_id: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

// ==================== ApiResponseTrait实现 ====================

impl ApiResponseTrait for SearchPersonsResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdatePersonResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdatePersonStatusResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetPersonAvatarResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for UploadPersonAvatarResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetPersonBasicInfoResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}
