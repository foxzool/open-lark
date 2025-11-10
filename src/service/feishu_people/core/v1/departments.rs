#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Departments v1 - 部门管理API
//!
//! 提供完整的部门生命周期管理功能，包括：
//! - 部门信息的增删改查操作
//! - 部门层级结构管理和维护
//! - 部门人员配置和组织关系
//! - 部门搜索和高级筛选功能
//! - 部门统计分析和报表
//! - 部门权限和配置管理
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::feishu_people::core::v1::departments::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 获取部门详情
//!     let response = client.feishu_people.core.v1.departments
//!         .get_department_builder("dept_001")
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.departments)
//!         .await?;
//!
//!     println!("部门名称: {}", response.data.department.name);
//!
//!     // 创建新部门
//!     let department_data = DepartmentCreateData {
//!         name: "研发部".to_string(),
//!         en_name: Some("R&D Department".to_string()),
//!         parent_department_id: Some("dept_root".to_string()),
//!         description: Some("负责产品研发和技术创新".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let create_response = client.feishu_people.core.v1.departments
//!         .create_department_builder()
//!         .department_data(department_data)
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.departments)
//!         .await?;
//!
//!     println!("部门创建成功，ID: {}", create_response.data.department_id);
//!
//!     // 搜索部门
//!     let search_response = client.feishu_people.core.v1.departments
//!         .search_departments_builder()
//!         .query("研发")
//!         .page_size(20)
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.departments)
//!         .await?;
//!
//!     println!("搜索到 {} 个部门", search_response.data.items.len());
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
    api_req::ApiRequest, // trait_system::ExecutableBuilder temporarily disabled
    error::LarkAPIError as CoreLarkAPIError,
    SDKResult as CoreSDKResult,
};
use serde::{Deserialize, Serialize};

/// 部门管理服务
///
/// 提供完整的部门生命周期管理功能，包括部门的创建、更新、删除、查询等操作。
/// 支持企业级的部门管理需求，包括层级结构、批量操作、高级搜索和统计分析功能。
///
/// # 核心功能
///
/// - **部门CRUD操作**: 创建、查询、更新、删除部门信息
/// - **层级结构管理**: 部门树形结构和父子关系维护
/// - **批量处理**: 支持批量获取和操作部门数据
/// - **高级搜索**: 基于多种条件的部门搜索和筛选
/// - **统计分析**: 部门人员、结构等统计信息
/// - **人员管理**: 部门人员配置和关系维护
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::feishu_people::core::v1::departments::DepartmentsService;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = DepartmentsService::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct DepartmentsService {
    pub config: Config,
}

impl DepartmentsService {
    /// 创建部门服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::DepartmentsService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = DepartmentsService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取部门详情
    ///
    /// 根据部门ID获取部门的详细信息，包括基本信息、人员配置、层级结构等。
    /// 支持使用不同的用户ID类型进行查询，返回完整的部门配置信息。
    ///
    /// # API文档
    ///
    /// 根据部门ID获取部门的详细信息。
    /// 返回部门的基本信息、人员配置、层级结构、扩展属性等完整资料。
    ///
    /// # 参数
    ///
    /// * `department_id` - 部门ID
    /// * `request` - 获取部门的请求参数，包含ID类型配置
    ///
    /// # 返回值
    ///
    /// 返回部门的详细信息，包含人员、层级、扩展属性等完整数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let request = GetDepartmentRequest {
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .get("dept_001", &request).await?;
    /// println!("部门名称: {}", response.data.department.name);
    /// ```
    pub async fn get(
        &self,
        department_id: &str,
        request: &GetDepartmentRequest,
    ) -> SDKResult<BaseResponse<GetDepartmentResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/feishu_people/core/v1/departments/{}",
                department_id
            ),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 批量获取部门信息
    ///
    /// 根据部门ID列表批量获取多个部门的详细信息。
    /// 适用于需要同时查询多个部门信息的场景，提高查询效率。
    ///
    /// # API文档
    ///
    /// 根据部门ID列表批量获取部门信息。
    /// 最多支持50个部门ID的批量查询，返回部门的完整信息。
    ///
    /// # 参数
    ///
    /// * `request` - 批量获取部门的请求参数，包含部门ID列表和配置
    ///
    /// # 返回值
    ///
    /// 返回批量部门的详细信息列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let request = BatchGetDepartmentsRequest {
    ///     department_ids: vec!["dept_001".to_string(), "dept_002".to_string()],
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .batch_get(&request).await?;
    /// println!("获取到 {} 个部门", response.data.items.len());
    /// ```
    pub async fn batch_get(
        &self,
        request: &BatchGetDepartmentsRequest,
    ) -> SDKResult<BaseResponse<BatchGetDepartmentsResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/departments/batch_get".to_string(),
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取子部门列表
    ///
    /// 获取指定部门的下级部门列表，支持分页查询。
    /// 可以用于了解部门的组织架构和下级配置情况。
    ///
    /// # API文档
    ///
    /// 根据部门ID获取该部门的直接下级部门列表。
    /// 支持分页查询，返回部门的直接子部门信息。
    ///
    /// # 参数
    ///
    /// * `request` - 获取子部门列表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回子部门列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let request = GetSubDepartmentsRequest {
    ///     department_id: "dept_root".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .get_sub_departments(&request).await?;
    /// println!("子部门数量: {}", response.data.items.len());
    /// ```
    pub async fn get_sub_departments(
        &self,
        request: &GetSubDepartmentsRequest,
    ) -> SDKResult<BaseResponse<GetSubDepartmentsResponse>> {
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
            api_path: "/open-apis/feishu_people/core/v1/departments/sub_departments".to_string(),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取根部门列表
    ///
    /// 获取组织架构中的根部门列表，即顶级部门。
    /// 可以用于了解组织的顶层结构和主要业务单元。
    ///
    /// # API文档
    ///
    /// 获取组织中的根部门（顶级部门）列表。
    /// 支持分页查询，返回组织架构的最高层级部门。
    ///
    /// # 参数
    ///
    /// * `request` - 获取根部门列表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回根部门列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let request = GetRootDepartmentsRequest {
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .get_root_departments(&request).await?;
    /// println!("根部门数量: {}", response.data.items.len());
    /// ```
    pub async fn get_root_departments(
        &self,
        request: &GetRootDepartmentsRequest,
    ) -> SDKResult<BaseResponse<GetRootDepartmentsResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
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
            api_path: "/open-apis/feishu_people/core/v1/departments/root_departments".to_string(),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 搜索部门
    ///
    /// 根据关键词搜索部门，支持按部门名称、描述、标签等字段进行搜索。
    /// 返回匹配的部门列表，支持分页查询和相关性排序。
    ///
    /// # API文档
    ///
    /// 根据关键词搜索部门信息。
    /// 支持按部门名称、英文名称、描述、标签等多种字段进行模糊搜索。
    ///
    /// # 参数
    ///
    /// * `request` - 搜索部门的请求参数，包含搜索关键词和过滤条件
    ///
    /// # 返回值
    ///
    /// 返回搜索结果和分页信息，包含匹配度评分
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let request = SearchDepartmentsRequest {
    ///     query: "研发".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .search(&request).await?;
    /// println!("搜索到 {} 个部门", response.data.items.len());
    /// ```
    pub async fn search(
        &self,
        request: &SearchDepartmentsRequest,
    ) -> SDKResult<BaseResponse<SearchDepartmentsResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/departments/search".to_string(),
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 创建部门
    ///
    /// 创建新的部门，设置部门的基本信息、层级关系、负责人等。
    /// 创建成功后，部门将可用于组织架构管理和人员配置。
    ///
    /// # API文档
    ///
    /// 创建新的部门信息，系统会自动分配部门ID。
    /// 支持设置部门的名称、层级、负责人、描述等完整信息。
    ///
    /// # 参数
    ///
    /// * `request` - 创建部门的请求参数，包含部门信息和配置
    ///
    /// # 返回值
    ///
    /// 返回创建成功的部门信息，包含系统分配的部门ID
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let department_data = DepartmentCreateData {
    ///     name: "研发部".to_string(),
    ///     en_name: Some("R&D Department".to_string()),
    ///     parent_department_id: Some("dept_root".to_string()),
    ///     leader_user_id: Some("user_001".to_string()),
    ///     description: Some("负责产品研发和技术创新".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = CreateDepartmentRequest {
    ///     department_data,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .create(&request).await?;
    /// println!("部门创建成功，ID: {}", response.data.department_id);
    /// ```
    pub async fn create(
        &self,
        request: &CreateDepartmentRequest,
    ) -> SDKResult<BaseResponse<CreateDepartmentResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/departments".to_string(),
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 更新部门信息
    ///
    /// 更新部门的信息，支持修改部门的基本属性、层级关系等。
    /// 只更新传入的字段，未传入的字段保持不变。
    ///
    /// # API文档
    ///
    /// 修改部门的部分信息，只更新提供的字段。
    /// 支持修改部门的名称、描述、负责人、状态等信息。
    ///
    /// # 参数
    ///
    /// * `department_id` - 部门ID
    /// * `request` - 修改部门的请求参数
    ///
    /// # 返回值
    ///
    /// 返回修改后的部门信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let update_data = DepartmentUpdateData {
    ///     name: Some("研发部(更新)".to_string()),
    ///     description: Some("负责产品研发、技术创新和架构设计".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = UpdateDepartmentRequest {
    ///     department_data: update_data,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .update("dept_001", &request).await?;
    /// println!("部门信息更新成功");
    /// ```
    pub async fn update(
        &self,
        department_id: &str,
        request: &UpdateDepartmentRequest,
    ) -> SDKResult<BaseResponse<UpdateDepartmentResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: format!(
                "/open-apis/feishu_people/core/v1/departments/{}",
                department_id
            ),
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 删除部门
    ///
    /// 删除指定的部门，删除后部门将不再可用。
    /// 删除操作不可逆，请谨慎使用。建议先停用部门再删除。
    ///
    /// # API文档
    ///
    /// 删除部门信息，操作不可逆。
    /// 删除前请确保部门下没有子部门和人员，相关数据将被清理。
    ///
    /// # 参数
    ///
    /// * `department_id` - 要删除的部门ID
    /// * `request` - 删除部门的请求参数
    ///
    /// # 返回值
    ///
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let request = DeleteDepartmentRequest {
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .delete("dept_001", &request).await?;
    /// println!("部门删除成功");
    /// ```
    pub async fn delete(
        &self,
        department_id: &str,
        request: &DeleteDepartmentRequest,
    ) -> SDKResult<BaseResponse<DeleteDepartmentResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!(
                "/open-apis/feishu_people/core/v1/departments/{}",
                department_id
            ),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取部门人员列表
    ///
    /// 获取指定部门的人员列表，包括当前和历史的人员配置情况。
    /// 支持分页查询，用于了解部门的人员配置和组织关系。
    ///
    /// # API文档
    ///
    /// 获取部门的人员配置列表，支持分页查询。
    /// 返回部门的人员信息、职位、状态等详细数据。
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
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let request = GetDepartmentMembersRequest {
    ///     department_id: "dept_001".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .get_members(&request).await?;
    /// println!("部门人员数量: {}", response.data.items.len());
    /// ```
    pub async fn get_members(
        &self,
        request: &GetDepartmentMembersRequest,
    ) -> SDKResult<BaseResponse<GetDepartmentMembersResponse>> {
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
            api_path: "/open-apis/feishu_people/core/v1/departments/members".to_string(),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取部门统计信息
    ///
    /// 获取部门的统计分析数据，包括人员统计、层级统计、变动统计等。
    /// 支持按时间范围过滤，用于组织架构分析和人力资源规划。
    ///
    /// # API文档
    ///
    /// 获取部门统计信息，支持按时间范围过滤。
    /// 返回部门的人员配置、层级结构、变动情况等统计数据。
    ///
    /// # 参数
    ///
    /// * `request` - 获取部门统计信息的请求参数
    ///
    /// # 返回值
    ///
    /// 返回部门的详细统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let request = GetDepartmentStatisticsRequest {
    ///     department_id: "dept_001".to_string(),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.departments
    ///     .get_statistics(&request).await?;
    /// println!("总成员数: {}", response.data.statistics.total_members);
    /// println!("活跃率: {:.2}%", response.data.statistics.active_rate * 100.0);
    /// ```
    pub async fn get_statistics(
        &self,
        request: &GetDepartmentStatisticsRequest,
    ) -> SDKResult<BaseResponse<GetDepartmentStatisticsResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("department_id", request.department_id.clone());

        // 构建API请求
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/feishu_people/core/v1/departments/statistics".to_string(),
            body: Vec::new(),
            query_params,
            ..Default::default()
        };
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    // ==================== Builder模式实现 ====================

    /// 获取部门详情构建器
    ///
    /// 提供流式API来构建获取部门详情的请求参数。
    /// 支持链式调用，方便配置查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .get_department_builder("dept_001")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn get_department_builder(&self, department_id: &str) -> GetDepartmentBuilder {
        GetDepartmentBuilder::new(department_id)
    }

    /// 批量获取部门构建器
    ///
    /// 提供流式API来构建批量获取部门的请求参数。
    /// 支持链式调用，方便配置批量查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .batch_get_departments_builder()
    ///     .department_ids(vec!["dept_001".to_string(), "dept_002".to_string()])
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn batch_get_departments_builder(&self) -> BatchGetDepartmentsBuilder {
        BatchGetDepartmentsBuilder::new()
    }

    /// 获取子部门列表构建器
    ///
    /// 提供流式API来构建获取子部门列表的请求参数。
    /// 支持链式调用，方便配置分页和过滤参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .get_sub_departments_builder("dept_root")
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn get_sub_departments_builder(&self, department_id: &str) -> GetSubDepartmentsBuilder {
        GetSubDepartmentsBuilder::new(department_id)
    }

    /// 获取根部门列表构建器
    ///
    /// 提供流式API来构建获取根部门列表的请求参数。
    /// 支持链式调用，方便配置分页参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .get_root_departments_builder()
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn get_root_departments_builder(&self) -> GetRootDepartmentsBuilder {
        GetRootDepartmentsBuilder::new()
    }

    /// 搜索部门构建器
    ///
    /// 提供流式API来构建搜索部门的请求参数。
    /// 支持链式调用，方便配置搜索条件和分页参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .search_departments_builder()
    ///     .query("研发".to_string())
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn search_departments_builder(&self) -> SearchDepartmentsBuilder {
        SearchDepartmentsBuilder::new()
    }

    /// 创建部门构建器
    ///
    /// 提供流式API来构建创建部门的请求参数。
    /// 支持链式调用，方便构建复杂的部门创建请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let department_data = DepartmentCreateData {
    ///     name: "研发部".to_string(),
    ///     parent_department_id: Some("dept_root".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .create_department_builder()
    ///     .department_data(department_data)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn create_department_builder(&self) -> CreateDepartmentBuilder {
        CreateDepartmentBuilder::new()
    }

    /// 更新部门构建器
    ///
    /// 提供流式API来构建更新部门的请求参数。
    /// 支持链式调用，方便构建部门更新请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let update_data = DepartmentUpdateData {
    ///     name: Some("研发部(更新)".to_string()),
    ///     description: Some("新的描述".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .update_department_builder("dept_001")
    ///     .department_data(update_data)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn update_department_builder(&self, department_id: &str) -> UpdateDepartmentBuilder {
        UpdateDepartmentBuilder::new(department_id)
    }

    /// 删除部门构建器
    ///
    /// 提供流式API来构建删除部门的请求参数。
    /// 支持链式调用，方便配置删除参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .delete_department_builder("dept_001")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn delete_department_builder(&self, department_id: &str) -> DeleteDepartmentBuilder {
        DeleteDepartmentBuilder::new(department_id)
    }

    /// 获取部门人员列表构建器
    ///
    /// 提供流式API来构建获取部门人员列表的请求参数。
    /// 支持链式调用，方便配置分页和查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .get_department_members_builder("dept_001")
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn get_department_members_builder(
        &self,
        department_id: &str,
    ) -> GetDepartmentMembersBuilder {
        GetDepartmentMembersBuilder::new(department_id)
    }

    /// 获取部门统计信息构建器
    ///
    /// 提供流式API来构建获取部门统计信息的请求参数。
    /// 支持链式调用，方便配置过滤参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::departments::*;
    ///
    /// let builder = client.feishu_people.core.v1.departments
    ///     .get_department_statistics_builder("dept_001");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.departments).await?;
    /// ```
    pub fn get_department_statistics_builder(
        &self,
        department_id: &str,
    ) -> GetDepartmentStatisticsBuilder {
        GetDepartmentStatisticsBuilder::new(department_id)
    }
}

// ==================== Builder结构体实现 ====================

/// 获取部门详情构建器
#[derive(Debug, Clone)]
pub struct GetDepartmentBuilder {
    department_id: String,
    request: GetDepartmentRequest,
}

impl GetDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new(department_id: &str) -> Self {
        Self {
            department_id: department_id.to_string(),
            request: GetDepartmentRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, GetDepartmentRequest) {
        (self.department_id, self.request)
    }
}

impl Default for GetDepartmentBuilder {
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
impl open_lark_core::trait_system::ExecutableBuilder<
    DepartmentsService,
    (String, GetDepartmentRequest),
    BaseResponse<GetDepartmentResponse>,
> for GetDepartmentBuilder
{
    fn build(self) -> (String, GetDepartmentRequest) {
        (self.department_id, self.request)
    }

    async fn execute(
        self,
        service: &DepartmentsService,
    ) -> CoreSDKResult<BaseResponse<GetDepartmentResponse>> {
        service
            .get_with_tuple((self.department_id, self.request))
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &DepartmentsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<GetDepartmentResponse>> {
        service
            .get_with_tuple((self.department_id, self.request))
            .await
            .map_err(map_sdk_error)
    }
}

/// 批量获取部门构建器
#[derive(Debug, Clone)]
pub struct BatchGetDepartmentsBuilder {
    request: BatchGetDepartmentsRequest,
}

impl BatchGetDepartmentsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: BatchGetDepartmentsRequest::default(),
        }
    }

    /// 设置部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request.department_ids = department_ids;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> BatchGetDepartmentsRequest {
        self.request
    }
}

impl Default for BatchGetDepartmentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait - 标准实现
// crate::impl_executable_builder!(
//    BatchGetDepartmentsBuilder,
//    DepartmentsService,
//    BatchGetDepartmentsRequest,
//    BaseResponse<BatchGetDepartmentsResponse>,
//    batch_get
//);

/// 获取子部门列表构建器
#[derive(Debug, Clone)]
pub struct GetSubDepartmentsBuilder {
    request: GetSubDepartmentsRequest,
}

impl GetSubDepartmentsBuilder {
    /// 创建新的Builder实例
    pub fn new(department_id: &str) -> Self {
        Self {
            request: GetSubDepartmentsRequest {
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
    pub fn build(self) -> GetSubDepartmentsRequest {
        self.request
    }
}

impl Default for GetSubDepartmentsBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetSubDepartmentsBuilder,
//    DepartmentsService,
//    GetSubDepartmentsRequest,
//    BaseResponse<GetSubDepartmentsResponse>,
//    get_sub_departments
//);

/// 获取根部门列表构建器
#[derive(Debug, Clone)]
pub struct GetRootDepartmentsBuilder {
    request: GetRootDepartmentsRequest,
}

impl GetRootDepartmentsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetRootDepartmentsRequest::default(),
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
    pub fn build(self) -> GetRootDepartmentsRequest {
        self.request
    }
}

impl Default for GetRootDepartmentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetRootDepartmentsBuilder,
//    DepartmentsService,
//    GetRootDepartmentsRequest,
//    BaseResponse<GetRootDepartmentsResponse>,
//    get_root_departments
//);

/// 搜索部门构建器
#[derive(Debug, Clone)]
pub struct SearchDepartmentsBuilder {
    request: SearchDepartmentsRequest,
}

impl SearchDepartmentsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchDepartmentsRequest::default(),
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
    pub fn build(self) -> SearchDepartmentsRequest {
        self.request
    }
}

impl Default for SearchDepartmentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchDepartmentsBuilder,
//    DepartmentsService,
//    SearchDepartmentsRequest,
//    BaseResponse<SearchDepartmentsResponse>,
//    search
//);

/// 创建部门构建器
#[derive(Debug, Clone)]
pub struct CreateDepartmentBuilder {
    request: CreateDepartmentRequest,
}

impl CreateDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: CreateDepartmentRequest::default(),
        }
    }

    /// 设置部门数据
    pub fn department_data(mut self, department_data: DepartmentCreateData) -> Self {
        self.request.department_data = department_data;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreateDepartmentRequest {
        self.request
    }
}

impl Default for CreateDepartmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    CreateDepartmentBuilder,
//    DepartmentsService,
//    CreateDepartmentRequest,
//    BaseResponse<CreateDepartmentResponse>,
//    create
//);

/// 更新部门构建器
#[derive(Debug, Clone)]
pub struct UpdateDepartmentBuilder {
    department_id: String,
    request: UpdateDepartmentRequest,
}

impl UpdateDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new(department_id: &str) -> Self {
        Self {
            department_id: department_id.to_string(),
            request: UpdateDepartmentRequest::default(),
        }
    }

    /// 设置部门数据
    pub fn department_data(mut self, department_data: DepartmentUpdateData) -> Self {
        self.request.department_data = department_data;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, UpdateDepartmentRequest) {
        (self.department_id, self.request)
    }
}

impl Default for UpdateDepartmentBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        DepartmentsService,
        (String, UpdateDepartmentRequest),
        BaseResponse<UpdateDepartmentResponse>,
    > for UpdateDepartmentBuilder
{
    fn build(self) -> (String, UpdateDepartmentRequest) {
        (self.department_id, self.request)
    }

    async fn execute(
        self,
        service: &DepartmentsService,
    ) -> CoreSDKResult<BaseResponse<UpdateDepartmentResponse>> {
        service
            .update_with_tuple((self.department_id, self.request))
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &DepartmentsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<UpdateDepartmentResponse>> {
        service
            .update_with_tuple((self.department_id, self.request))
            .await
            .map_err(map_sdk_error)
    }
}

/// 删除部门构建器
#[derive(Debug, Clone)]
pub struct DeleteDepartmentBuilder {
    department_id: String,
    request: DeleteDepartmentRequest,
}

impl DeleteDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new(department_id: &str) -> Self {
        Self {
            department_id: department_id.to_string(),
            request: DeleteDepartmentRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, DeleteDepartmentRequest) {
        (self.department_id, self.request)
    }
}

impl Default for DeleteDepartmentBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    // TODO: Fix this reference - open_lark_core::trait_system::ExecutableBuilder<
        DepartmentsService,
        (String, DeleteDepartmentRequest),
        BaseResponse<DeleteDepartmentResponse>,
    > for DeleteDepartmentBuilder
{
    fn build(self) -> (String, DeleteDepartmentRequest) {
        (self.department_id, self.request)
    }

    async fn execute(
        self,
        service: &DepartmentsService,
    ) -> CoreSDKResult<BaseResponse<DeleteDepartmentResponse>> {
        service
            .delete_with_tuple((self.department_id, self.request))
            .await
            .map_err(map_sdk_error)
    }

    async fn execute_with_options(
        self,
        service: &DepartmentsService,
        _option: open_lark_core::req_option::RequestOption,
    ) -> CoreSDKResult<BaseResponse<DeleteDepartmentResponse>> {
        service
            .delete_with_tuple((self.department_id, self.request))
            .await
            .map_err(map_sdk_error)
    }
}

/// 获取部门人员列表构建器
#[derive(Debug, Clone)]
pub struct GetDepartmentMembersBuilder {
    request: GetDepartmentMembersRequest,
}

impl GetDepartmentMembersBuilder {
    /// 创建新的Builder实例
    pub fn new(department_id: &str) -> Self {
        Self {
            request: GetDepartmentMembersRequest {
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
    pub fn build(self) -> GetDepartmentMembersRequest {
        self.request
    }
}

impl Default for GetDepartmentMembersBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetDepartmentMembersBuilder,
//    DepartmentsService,
//    GetDepartmentMembersRequest,
//    BaseResponse<GetDepartmentMembersResponse>,
//    get_members
//);

/// 获取部门统计信息构建器
#[derive(Debug, Clone)]
pub struct GetDepartmentStatisticsBuilder {
    request: GetDepartmentStatisticsRequest,
}

impl GetDepartmentStatisticsBuilder {
    /// 创建新的Builder实例
    pub fn new(department_id: &str) -> Self {
        Self {
            request: GetDepartmentStatisticsRequest {
                department_id: department_id.to_string(),
            },
        }
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetDepartmentStatisticsRequest {
        self.request
    }
}

impl Default for GetDepartmentStatisticsBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetDepartmentStatisticsBuilder,
//    DepartmentsService,
//    GetDepartmentStatisticsRequest,
//    BaseResponse<GetDepartmentStatisticsResponse>,
//    get_statistics
//);

// 为DepartmentsService实现辅助方法，处理Builder的参数
impl DepartmentsService {
    async fn get_with_tuple(
        &self,
        params: (String, GetDepartmentRequest),
    ) -> SDKResult<BaseResponse<GetDepartmentResponse>> {
        self.get(&params.0, &params.1).await
    }

    async fn update_with_tuple(
        &self,
        params: (String, UpdateDepartmentRequest),
    ) -> SDKResult<BaseResponse<UpdateDepartmentResponse>> {
        self.update(&params.0, &params.1).await
    }

    async fn delete_with_tuple(
        &self,
        params: (String, DeleteDepartmentRequest),
    ) -> SDKResult<BaseResponse<DeleteDepartmentResponse>> {
        self.delete(&params.0, &params.1).await
    }
}

// ==================== 数据模型 ====================

/// 获取部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for GetDepartmentRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 获取部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for GetDepartmentResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

/// 批量获取部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetDepartmentsRequest {
    /// 部门ID列表
    pub department_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for BatchGetDepartmentsRequest {
    fn default() -> Self {
        Self {
            department_ids: vec![],
            user_id_type: None,
        }
    }
}

/// 批量获取部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetDepartmentsResponse {
    /// 部门列表
    pub items: Vec<Department>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for BatchGetDepartmentsResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

/// 获取子部门列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubDepartmentsRequest {
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

/// 获取子部门列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubDepartmentsResponse {
    /// 子部门列表
    pub items: Vec<Department>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for GetSubDepartmentsResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

/// 获取根部门列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRootDepartmentsRequest {
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

impl Default for GetRootDepartmentsRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }
}

/// 获取根部门列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRootDepartmentsResponse {
    /// 根部门列表
    pub items: Vec<Department>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for GetRootDepartmentsResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

/// 搜索部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDepartmentsRequest {
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

impl Default for SearchDepartmentsRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }
}

/// 搜索部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDepartmentsResponse {
    /// 搜索结果项
    pub items: Vec<DepartmentSearchResult>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 总数
    pub total: Option<i32>,
}

/// 创建部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentRequest {
    /// 部门数据
    pub department_data: DepartmentCreateData,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for CreateDepartmentRequest {
    fn default() -> Self {
        Self {
            department_data: DepartmentCreateData::default(),
            user_id_type: None,
        }
    }
}

/// 创建部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentResponse {
    /// 部门ID
    pub department_id: String,
    /// 部门信息
    pub department: Department,
}

/// 更新部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentRequest {
    /// 部门数据
    pub department_data: DepartmentUpdateData,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for UpdateDepartmentRequest {
    fn default() -> Self {
        Self {
            department_data: DepartmentUpdateData::default(),
            user_id_type: None,
        }
    }
}

/// 更新部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

/// 删除部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for DeleteDepartmentRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 删除部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteDepartmentResponse {}

/// 获取部门人员列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentMembersRequest {
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

/// 获取部门人员列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentMembersResponse {
    /// 人员列表
    pub items: Vec<Person>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for GetDepartmentMembersResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

/// 获取部门统计信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentStatisticsRequest {
    /// 部门ID
    pub department_id: String,
}

/// 获取部门统计信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentStatisticsResponse {
    /// 统计信息
    pub statistics: DepartmentStatistics,
}

/// 部门搜索结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepartmentSearchResult {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 父部门名称
    pub parent_department_name: Option<String>,
    /// 负责人姓名
    pub leader_name: Option<String>,
    /// 成员数量
    pub member_count: i32,
    /// 匹配分数
    pub match_score: f64,
}

/// 部门统计信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepartmentStatistics {
    /// 部门ID
    pub department_id: String,
    /// 总成员数
    pub total_members: i32,
    /// 活跃成员数
    pub active_members: i32,
    /// 非活跃成员数
    pub inactive_members: i32,
    /// 子部门数量
    pub sub_departments_count: i32,
    /// 近期入职人数
    pub recent_joins: i32,
    /// 近期离职人数
    pub recent_leaves: i32,
    /// 活跃率
    pub active_rate: f64,
    /// 性别分布
    pub gender_distribution: Option<serde_json::Value>,
    /// 年龄分布
    pub age_distribution: Option<serde_json::Value>,
}

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Department {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 部门负责人用户ID
    pub leader_user_id: Option<String>,
    /// 部门状态
    pub status: Option<String>,
    /// 成员数量
    pub member_count: Option<i32>,
    /// 部门描述
    pub description: Option<String>,
    /// 部门路径
    pub department_path: Option<Vec<String>>,
    /// 部门标签
    pub tags: Option<Vec<String>>,
    /// 创建时间
    pub created_at: Option<String>,
    /// 更新时间
    pub updated_at: Option<String>,
    /// 部门类型
    pub department_type: Option<String>,
    /// 部门级别
    pub level: Option<i32>,
    /// 排序权重
    pub order_weight: Option<i32>,
    /// 联系电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

/// 部门创建数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepartmentCreateData {
    /// 部门名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 部门负责人用户ID
    pub leader_user_id: Option<String>,
    /// 部门描述
    pub description: Option<String>,
    /// 部门标签
    pub tags: Option<Vec<String>>,
    /// 部门类型
    pub department_type: Option<String>,
    /// 联系电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

/// 部门更新数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepartmentUpdateData {
    /// 部门名称
    pub name: Option<String>,
    /// 英文名称
    pub en_name: Option<String>,
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 部门负责人用户ID
    pub leader_user_id: Option<String>,
    /// 部门状态
    pub status: Option<String>,
    /// 部门描述
    pub description: Option<String>,
    /// 部门标签
    pub tags: Option<Vec<String>>,
    /// 部门类型
    pub department_type: Option<String>,
    /// 排序权重
    pub order_weight: Option<i32>,
    /// 联系电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
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
    /// 员工类型
    pub employee_type: Option<String>,
    /// 状态
    pub status: Option<String>,
    /// 职位
    pub position: Option<String>,
    /// 部门
    pub department: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 入职时间
    pub hire_time: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

// ==================== ApiResponseTrait实现 ====================

impl ApiResponseTrait for SearchDepartmentsResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateDepartmentResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateDepartmentResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteDepartmentResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetDepartmentStatisticsResponse {
    fn data_format() -> open_lark_core::api_resp::ResponseFormat {
        open_lark_core::api_resp::ResponseFormat::Data
    }
}
