//! Companies v1 - 公司管理API
//!
//! 提供完整的公司生命周期管理功能，包括：
//! - 公司信息的增删改查操作
//! - 公司架构管理和层级维护
//! - 企业信息维护和统计分析
//! - 公司搜索和高级筛选功能
//! - 子公司管理和股权结构
//! - 组织架构和人员配置
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::feishu_people::core::v1::companies::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 获取公司详情
//!     let response = client.feishu_people.core.v1.companies
//!         .get_company_builder("company_001")
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.companies)
//!         .await?;
//!
//!     println!("公司名称: {}", response.data.company.name);
//!
//!     // 创建新公司
//!     let company_data = CompanyCreateData {
//!         name: "示例科技有限公司".to_string(),
//!         en_name: Some("Example Technology Co., Ltd.".to_string()),
//!         unified_social_credit_code: Some("91110000MA00XXXXXX".to_string()),
//!         legal_representative: Some("张总".to_string()),
//!         registered_capital: Some("1000万".to_string()),
//!         industry: Some("软件和信息技术服务业".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let create_response = client.feishu_people.core.v1.companies
//!         .create_company_builder()
//!         .company_data(company_data)
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.companies)
//!         .await?;
//!
//!     println!("公司创建成功，ID: {}", create_response.data.company_id);
//!
//!     // 搜索公司
//!     let search_response = client.feishu_people.core.v1.companies
//!         .search_companies_builder()
//!         .query("科技")
//!         .page_size(20)
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.companies)
//!         .await?;
//!
//!     println!("搜索到 {} 个公司", search_response.data.items.len());
//!
//!     Ok(())
//! }
//! ```

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
};

// Use open_lark_core's error type for compatibility with async traits
use crate::core::error::LarkAPIError;
pub type SDKResult<T> = Result<T, LarkAPIError>;
use open_lark_core::core::api_req::ApiRequest; // trait_system::ExecutableBuilder temporarily disabled
use serde::{Deserialize, Serialize};

/// 公司管理服务
///
/// 提供完整的公司生命周期管理功能，包括公司的创建、更新、删除、查询等操作。
/// 支持企业级的公司管理需求，包括架构管理、批量操作、高级搜索和统计分析功能。
///
/// # 核心功能
///
/// - **公司CRUD操作**: 创建、查询、更新、删除公司信息
/// - **架构管理**: 公司层级结构和股权关系维护
/// - **批量处理**: 支持批量获取和操作公司数据
/// - **高级搜索**: 基于多种条件的公司搜索和筛选
/// - **统计分析**: 公司人员、结构等统计信息
/// - **子公司管理**: 子公司信息和股权结构管理
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::feishu_people::core::v1::companies::CompaniesService;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = CompaniesService::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct CompaniesService {
    pub config: Config,
}

impl CompaniesService {
    /// 创建公司服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::CompaniesService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = CompaniesService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取公司详情
    ///
    /// 根据公司ID获取公司的详细信息，包括基本信息、工商信息、股权结构等。
    /// 支持使用不同的用户ID类型进行查询，返回完整的公司配置信息。
    ///
    /// # API文档
    ///
    /// 根据公司ID获取公司的详细信息。
    /// 返回公司的基本信息、工商信息、股权结构、扩展属性等完整资料。
    ///
    /// # 参数
    ///
    /// * `company_id` - 公司ID
    /// * `request` - 获取公司的请求参数，包含ID类型配置
    ///
    /// # 返回值
    ///
    /// 返回公司的详细信息，包含工商、股权、扩展属性等完整数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let request = GetCompanyRequest {
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .get("company_001", &request).await?;
    /// println!("公司名称: {}", response.data.company.name);
    /// ```
    pub async fn get(
        &self,
        company_id: &str,
        request: &GetCompanyRequest,
    ) -> SDKResult<BaseResponse<GetCompanyResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/feishu_people/core/v1/companies/{}", company_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 批量获取公司信息
    ///
    /// 根据公司ID列表批量获取多个公司的详细信息。
    /// 适用于需要同时查询多个公司信息的场景，提高查询效率。
    ///
    /// # API文档
    ///
    /// 根据公司ID列表批量获取公司信息。
    /// 最多支持50个公司ID的批量查询，返回公司的完整信息。
    ///
    /// # 参数
    ///
    /// * `request` - 批量获取公司的请求参数，包含公司ID列表和配置
    ///
    /// # 返回值
    ///
    /// 返回批量公司的详细信息列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let request = BatchGetCompaniesRequest {
    ///     company_ids: vec!["company_001".to_string(), "company_002".to_string()],
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .batch_get(&request).await?;
    /// println!("获取到 {} 个公司", response.data.items.len());
    /// ```
    pub async fn batch_get(
        &self,
        request: &BatchGetCompaniesRequest,
    ) -> SDKResult<BaseResponse<BatchGetCompaniesResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/companies/batch_get".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 搜索公司
    ///
    /// 根据关键词搜索公司，支持按公司名称、英文名称、行业等字段进行搜索。
    /// 返回匹配的公司列表，支持分页查询和相关性排序。
    ///
    /// # API文档
    ///
    /// 根据关键词搜索公司信息。
    /// 支持按公司名称、英文名称、统一社会信用代码、行业等多种字段进行模糊搜索。
    ///
    /// # 参数
    ///
    /// * `request` - 搜索公司的请求参数，包含搜索关键词和过滤条件
    ///
    /// # 返回值
    ///
    /// 返回搜索结果和分页信息，包含匹配度评分
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let request = SearchCompaniesRequest {
    ///     query: "科技".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .search(&request).await?;
    /// println!("搜索到 {} 个公司", response.data.items.len());
    /// ```
    pub async fn search(
        &self,
        request: &SearchCompaniesRequest,
    ) -> SDKResult<BaseResponse<SearchCompaniesResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/companies/search".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 创建公司
    ///
    /// 创建新的公司，设置公司的基本信息、工商信息、股权结构等。
    /// 创建成功后，公司将可用于组织架构管理和企业配置。
    ///
    /// # API文档
    ///
    /// 创建新的公司信息，系统会自动分配公司ID。
    /// 支持设置公司的名称、工商信息、股权结构、扩展属性等完整信息。
    ///
    /// # 参数
    ///
    /// * `request` - 创建公司的请求参数，包含公司信息和配置
    ///
    /// # 返回值
    ///
    /// 返回创建成功的公司信息，包含系统分配的公司ID
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let company_data = CompanyCreateData {
    ///     name: "示例科技有限公司".to_string(),
    ///     en_name: Some("Example Technology Co., Ltd.".to_string()),
    ///     unified_social_credit_code: Some("91110000MA00XXXXXX".to_string()),
    ///     legal_representative: Some("张总".to_string()),
    ///     registered_capital: Some("1000万".to_string()),
    ///     industry: Some("软件和信息技术服务业".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = CreateCompanyRequest {
    ///     company_data,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .create(&request).await?;
    /// println!("公司创建成功，ID: {}", response.data.company_id);
    /// ```
    pub async fn create(
        &self,
        request: &CreateCompanyRequest,
    ) -> SDKResult<BaseResponse<CreateCompanyResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/companies".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 更新公司信息
    ///
    /// 更新公司的信息，支持修改公司的基本属性、工商信息等。
    /// 只更新传入的字段，未传入的字段保持不变。
    ///
    /// # API文档
    ///
    /// 修改公司的部分信息，只更新提供的字段。
    /// 支持修改公司的名称、工商信息、股权结构、状态等信息。
    ///
    /// # 参数
    ///
    /// * `company_id` - 公司ID
    /// * `request` - 修改公司的请求参数
    ///
    /// # 返回值
    ///
    /// 返回修改后的公司信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let update_data = CompanyUpdateData {
    ///     name: Some("示例科技有限公司(更新)".to_string()),
    ///     registered_capital: Some("2000万".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = UpdateCompanyRequest {
    ///     company_data: update_data,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .update("company_001", &request).await?;
    /// println!("公司信息更新成功");
    /// ```
    pub async fn update(
        &self,
        company_id: &str,
        request: &UpdateCompanyRequest,
    ) -> SDKResult<BaseResponse<UpdateCompanyResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::PUT,
            api_path: format!("/open-apis/feishu_people/core/v1/companies/{}", company_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 删除公司
    ///
    /// 删除指定的公司，删除后公司将不再可用。
    /// 删除操作不可逆，请谨慎使用。建议先停用公司再删除。
    ///
    /// # API文档
    ///
    /// 删除公司信息，操作不可逆。
    /// 删除前请确保公司下没有子公司和人员，相关数据将被清理。
    ///
    /// # 参数
    ///
    /// * `company_id` - 要删除的公司ID
    /// * `request` - 删除公司的请求参数
    ///
    /// # 返回值
    ///
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let request = DeleteCompanyRequest {
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .delete("company_001", &request).await?;
    /// println!("公司删除成功");
    /// ```
    pub async fn delete(
        &self,
        company_id: &str,
        request: &DeleteCompanyRequest,
    ) -> SDKResult<BaseResponse<DeleteCompanyResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::DELETE,
            api_path: format!("/open-apis/feishu_people/core/v1/companies/{}", company_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取公司统计信息
    ///
    /// 获取公司的统计分析数据，包括人员统计、结构统计、变动统计等。
    /// 支持按时间范围过滤，用于组织架构分析和人力资源规划。
    ///
    /// # API文档
    ///
    /// 获取公司统计信息，支持按时间范围过滤。
    /// 返回公司的人员配置、组织结构、变动情况等统计数据。
    ///
    /// # 参数
    ///
    /// * `request` - 获取公司统计信息的请求参数
    ///
    /// # 返回值
    ///
    /// 返回公司的详细统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let request = GetCompanyStatisticsRequest {
    ///     company_id: "company_001".to_string(),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .get_statistics(&request).await?;
    /// println!("总员工数: {}", response.data.statistics.total_employees);
    /// println!("活跃率: {:.2}%", response.data.statistics.active_rate * 100.0);
    /// ```
    pub async fn get_statistics(
        &self,
        request: &GetCompanyStatisticsRequest,
    ) -> SDKResult<BaseResponse<GetCompanyStatisticsResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("company_id", request.company_id.clone());

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: "/open-apis/feishu_people/core/v1/companies/statistics".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取公司组织架构
    ///
    /// 获取公司的完整组织架构信息，包括部门层级、人员配置等。
    /// 支持深度遍历，用于了解公司的组织结构和人员分布。
    ///
    /// # API文档
    ///
    /// 获取公司的组织架构信息，支持深度遍历。
    /// 返回公司的部门层级、人员配置、结构统计等完整数据。
    ///
    /// # 参数
    ///
    /// * `request` - 获取公司组织架构的请求参数
    ///
    /// # 返回值
    ///
    /// 返回公司的完整组织架构信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let request = GetOrganizationStructureRequest {
    ///     company_id: "company_001".to_string(),
    ///     max_depth: Some(5),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .get_organization_structure(&request).await?;
    /// println!("根部门数量: {}", response.data.structure.root_departments.len());
    /// println!("总部门数: {}", response.data.structure.total_departments);
    /// ```
    pub async fn get_organization_structure(
        &self,
        request: &GetOrganizationStructureRequest,
    ) -> SDKResult<BaseResponse<GetOrganizationStructureResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("company_id", request.company_id.clone());
        if let Some(max_depth) = request.max_depth {
            query_params.insert("max_depth", max_depth.to_string());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: "/open-apis/feishu_people/core/v1/companies/organization_structure"
                .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取子公司列表
    ///
    /// 获取指定公司的子公司列表，支持分页查询。
    /// 可以用于了解公司的股权结构和业务布局。
    ///
    /// # API文档
    ///
    /// 根据母公司ID获取其子公司列表。
    /// 支持分页查询，返回子公司的基本信息和股权关系。
    ///
    /// # 参数
    ///
    /// * `request` - 获取子公司列表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回子公司列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let request = GetSubsidiariesRequest {
    ///     parent_company_id: "company_001".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.companies
    ///     .get_subsidiaries(&request).await?;
    /// println!("子公司数量: {}", response.data.items.len());
    /// ```
    pub async fn get_subsidiaries(
        &self,
        request: &GetSubsidiariesRequest,
    ) -> SDKResult<BaseResponse<GetSubsidiariesResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("parent_company_id", request.parent_company_id.clone());
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
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: "/open-apis/feishu_people/core/v1/companies/subsidiaries".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    // ==================== Builder模式实现 ====================

    /// 获取公司详情构建器
    ///
    /// 提供流式API来构建获取公司详情的请求参数。
    /// 支持链式调用，方便配置查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .get_company_builder("company_001")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn get_company_builder(&self, company_id: &str) -> GetCompanyBuilder {
        GetCompanyBuilder::new(company_id)
    }

    /// 批量获取公司构建器
    ///
    /// 提供流式API来构建批量获取公司的请求参数。
    /// 支持链式调用，方便配置批量查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .batch_get_companies_builder()
    ///     .company_ids(vec!["company_001".to_string(), "company_002".to_string()])
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn batch_get_companies_builder(&self) -> BatchGetCompaniesBuilder {
        BatchGetCompaniesBuilder::new()
    }

    /// 搜索公司构建器
    ///
    /// 提供流式API来构建搜索公司的请求参数。
    /// 支持链式调用，方便配置搜索条件和分页参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .search_companies_builder()
    ///     .query("科技".to_string())
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn search_companies_builder(&self) -> SearchCompaniesBuilder {
        SearchCompaniesBuilder::new()
    }

    /// 创建公司构建器
    ///
    /// 提供流式API来构建创建公司的请求参数。
    /// 支持链式调用，方便构建复杂的公司创建请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let company_data = CompanyCreateData {
    ///     name: "示例科技有限公司".to_string(),
    ///     industry: Some("软件和信息技术服务业".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .create_company_builder()
    ///     .company_data(company_data)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn create_company_builder(&self) -> CreateCompanyBuilder {
        CreateCompanyBuilder::new()
    }

    /// 更新公司构建器
    ///
    /// 提供流式API来构建更新公司的请求参数。
    /// 支持链式调用，方便构建公司更新请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let update_data = CompanyUpdateData {
    ///     name: Some("示例科技有限公司(更新)".to_string()),
    ///     registered_capital: Some("2000万".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .update_company_builder("company_001")
    ///     .company_data(update_data)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn update_company_builder(&self, company_id: &str) -> UpdateCompanyBuilder {
        UpdateCompanyBuilder::new(company_id)
    }

    /// 删除公司构建器
    ///
    /// 提供流式API来构建删除公司的请求参数。
    /// 支持链式调用，方便配置删除参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .delete_company_builder("company_001")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn delete_company_builder(&self, company_id: &str) -> DeleteCompanyBuilder {
        DeleteCompanyBuilder::new(company_id)
    }

    /// 获取公司统计信息构建器
    ///
    /// 提供流式API来构建获取公司统计信息的请求参数。
    /// 支持链式调用，方便配置过滤参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .get_company_statistics_builder("company_001");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn get_company_statistics_builder(&self, company_id: &str) -> GetCompanyStatisticsBuilder {
        GetCompanyStatisticsBuilder::new(company_id)
    }

    /// 获取公司组织架构构建器
    ///
    /// 提供流式API来构建获取公司组织架构的请求参数。
    /// 支持链式调用，方便配置深度参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .get_organization_structure_builder("company_001")
    ///     .max_depth(5);
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn get_organization_structure_builder(
        &self,
        company_id: &str,
    ) -> GetOrganizationStructureBuilder {
        GetOrganizationStructureBuilder::new(company_id)
    }

    /// 获取子公司列表构建器
    ///
    /// 提供流式API来构建获取子公司列表的请求参数。
    /// 支持链式调用，方便配置分页和查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::companies::*;
    ///
    /// let builder = client.feishu_people.core.v1.companies
    ///     .get_subsidiaries_builder("company_001")
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.companies).await?;
    /// ```
    pub fn get_subsidiaries_builder(&self, parent_company_id: &str) -> GetSubsidiariesBuilder {
        GetSubsidiariesBuilder::new(parent_company_id)
    }
}

// ==================== Builder结构体实现 ====================

/// 获取公司详情构建器
#[derive(Debug, Clone)]
pub struct GetCompanyBuilder {
    company_id: String,
    request: GetCompanyRequest,
}

impl GetCompanyBuilder {
    /// 创建新的Builder实例
    pub fn new(company_id: &str) -> Self {
        Self {
            company_id: company_id.to_string(),
            request: GetCompanyRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, GetCompanyRequest) {
        (self.company_id, self.request)
    }
}

impl Default for GetCompanyBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    open_lark_core::core::trait_system::ExecutableBuilder<
        CompaniesService,
        (String, GetCompanyRequest),
        BaseResponse<GetCompanyResponse>,
    > for GetCompanyBuilder
{
    async fn execute(
        &self,
        service: &CompaniesService,
    ) -> SDKResult<BaseResponse<GetCompanyResponse>> {
        service.get_with_tuple(self.clone().build()).await
    }
}

/// 批量获取公司构建器
#[derive(Debug, Clone)]
pub struct BatchGetCompaniesBuilder {
    request: BatchGetCompaniesRequest,
}

impl BatchGetCompaniesBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: BatchGetCompaniesRequest::default(),
        }
    }

    /// 设置公司ID列表
    pub fn company_ids(mut self, company_ids: Vec<String>) -> Self {
        self.request.company_ids = company_ids;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> BatchGetCompaniesRequest {
        self.request
    }
}

impl Default for BatchGetCompaniesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait - 标准实现
// crate::impl_executable_builder!(
//    BatchGetCompaniesBuilder,
//    CompaniesService,
//    BatchGetCompaniesRequest,
//    BaseResponse<BatchGetCompaniesResponse>,
//    batch_get
//);

/// 搜索公司构建器
#[derive(Debug, Clone)]
pub struct SearchCompaniesBuilder {
    request: SearchCompaniesRequest,
}

impl SearchCompaniesBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchCompaniesRequest::default(),
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
    pub fn build(self) -> SearchCompaniesRequest {
        self.request
    }
}

impl Default for SearchCompaniesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchCompaniesBuilder,
//    CompaniesService,
//    SearchCompaniesRequest,
//    BaseResponse<SearchCompaniesResponse>,
//    search
//);

/// 创建公司构建器
#[derive(Debug, Clone)]
pub struct CreateCompanyBuilder {
    request: CreateCompanyRequest,
}

impl CreateCompanyBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: CreateCompanyRequest::default(),
        }
    }

    /// 设置公司数据
    pub fn company_data(mut self, company_data: CompanyCreateData) -> Self {
        self.request.company_data = company_data;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreateCompanyRequest {
        self.request
    }
}

impl Default for CreateCompanyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    CreateCompanyBuilder,
//    CompaniesService,
//    CreateCompanyRequest,
//    BaseResponse<CreateCompanyResponse>,
//    create
//);

/// 更新公司构建器
#[derive(Debug, Clone)]
pub struct UpdateCompanyBuilder {
    company_id: String,
    request: UpdateCompanyRequest,
}

impl UpdateCompanyBuilder {
    /// 创建新的Builder实例
    pub fn new(company_id: &str) -> Self {
        Self {
            company_id: company_id.to_string(),
            request: UpdateCompanyRequest::default(),
        }
    }

    /// 设置公司数据
    pub fn company_data(mut self, company_data: CompanyUpdateData) -> Self {
        self.request.company_data = company_data;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, UpdateCompanyRequest) {
        (self.company_id, self.request)
    }
}

impl Default for UpdateCompanyBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    open_lark_core::core::trait_system::ExecutableBuilder<
        CompaniesService,
        (String, UpdateCompanyRequest),
        BaseResponse<UpdateCompanyResponse>,
    > for UpdateCompanyBuilder
{
    async fn execute(
        &self,
        service: &CompaniesService,
    ) -> SDKResult<BaseResponse<UpdateCompanyResponse>> {
        service.update_with_tuple(self.clone().build()).await
    }
}

/// 删除公司构建器
#[derive(Debug, Clone)]
pub struct DeleteCompanyBuilder {
    company_id: String,
    request: DeleteCompanyRequest,
}

impl DeleteCompanyBuilder {
    /// 创建新的Builder实例
    pub fn new(company_id: &str) -> Self {
        Self {
            company_id: company_id.to_string(),
            request: DeleteCompanyRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, DeleteCompanyRequest) {
        (self.company_id, self.request)
    }
}

impl Default for DeleteCompanyBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    open_lark_core::core::trait_system::ExecutableBuilder<
        CompaniesService,
        (String, DeleteCompanyRequest),
        BaseResponse<DeleteCompanyResponse>,
    > for DeleteCompanyBuilder
{
    async fn execute(
        &self,
        service: &CompaniesService,
    ) -> SDKResult<BaseResponse<DeleteCompanyResponse>> {
        service.delete_with_tuple(self.clone().build()).await
    }
}

/// 获取公司统计信息构建器
#[derive(Debug, Clone)]
pub struct GetCompanyStatisticsBuilder {
    request: GetCompanyStatisticsRequest,
}

impl GetCompanyStatisticsBuilder {
    /// 创建新的Builder实例
    pub fn new(company_id: &str) -> Self {
        Self {
            request: GetCompanyStatisticsRequest {
                company_id: company_id.to_string(),
            },
        }
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetCompanyStatisticsRequest {
        self.request
    }
}

impl Default for GetCompanyStatisticsBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetCompanyStatisticsBuilder,
//    CompaniesService,
//    GetCompanyStatisticsRequest,
//    BaseResponse<GetCompanyStatisticsResponse>,
//    get_statistics
//);

/// 获取公司组织架构构建器
#[derive(Debug, Clone)]
pub struct GetOrganizationStructureBuilder {
    request: GetOrganizationStructureRequest,
}

impl GetOrganizationStructureBuilder {
    /// 创建新的Builder实例
    pub fn new(company_id: &str) -> Self {
        Self {
            request: GetOrganizationStructureRequest {
                company_id: company_id.to_string(),
                max_depth: None,
            },
        }
    }

    /// 设置最大深度
    pub fn max_depth(mut self, max_depth: i32) -> Self {
        self.request.max_depth = Some(max_depth);
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetOrganizationStructureRequest {
        self.request
    }
}

impl Default for GetOrganizationStructureBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetOrganizationStructureBuilder,
//    CompaniesService,
//    GetOrganizationStructureRequest,
//    BaseResponse<GetOrganizationStructureResponse>,
//    get_organization_structure
//);

/// 获取子公司列表构建器
#[derive(Debug, Clone)]
pub struct GetSubsidiariesBuilder {
    request: GetSubsidiariesRequest,
}

impl GetSubsidiariesBuilder {
    /// 创建新的Builder实例
    pub fn new(parent_company_id: &str) -> Self {
        Self {
            request: GetSubsidiariesRequest {
                parent_company_id: parent_company_id.to_string(),
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
    pub fn build(self) -> GetSubsidiariesRequest {
        self.request
    }
}

impl Default for GetSubsidiariesBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetSubsidiariesBuilder,
//    CompaniesService,
//    GetSubsidiariesRequest,
//    BaseResponse<GetSubsidiariesResponse>,
//    get_subsidiaries
//);

// 为CompaniesService实现辅助方法，处理Builder的参数
impl CompaniesService {
    async fn get_with_tuple(
        &self,
        params: (String, GetCompanyRequest),
    ) -> SDKResult<BaseResponse<GetCompanyResponse>> {
        self.get(&params.0, &params.1).await
    }

    async fn update_with_tuple(
        &self,
        params: (String, UpdateCompanyRequest),
    ) -> SDKResult<BaseResponse<UpdateCompanyResponse>> {
        self.update(&params.0, &params.1).await
    }

    async fn delete_with_tuple(
        &self,
        params: (String, DeleteCompanyRequest),
    ) -> SDKResult<BaseResponse<DeleteCompanyResponse>> {
        self.delete(&params.0, &params.1).await
    }
}

// ==================== 数据模型 ====================

/// 获取公司请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCompanyRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for GetCompanyRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 获取公司响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCompanyResponse {
    /// 公司信息
    pub company: Company,
}

impl ApiResponseTrait for GetCompanyResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 批量获取公司请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetCompaniesRequest {
    /// 公司ID列表
    pub company_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for BatchGetCompaniesRequest {
    fn default() -> Self {
        Self {
            company_ids: vec![],
            user_id_type: None,
        }
    }
}

/// 批量获取公司响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetCompaniesResponse {
    /// 公司列表
    pub items: Vec<Company>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for BatchGetCompaniesResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 搜索公司请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCompaniesRequest {
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

impl Default for SearchCompaniesRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }
}

/// 搜索公司响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCompaniesResponse {
    /// 搜索结果项
    pub items: Vec<CompanySearchResult>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for SearchCompaniesResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 创建公司请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompanyRequest {
    /// 公司数据
    pub company_data: CompanyCreateData,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for CreateCompanyRequest {
    fn default() -> Self {
        Self {
            company_data: CompanyCreateData::default(),
            user_id_type: None,
        }
    }
}

/// 创建公司响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompanyResponse {
    /// 公司ID
    pub company_id: String,
    /// 公司信息
    pub company: Company,
}

impl ApiResponseTrait for CreateCompanyResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 更新公司请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCompanyRequest {
    /// 公司数据
    pub company_data: CompanyUpdateData,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for UpdateCompanyRequest {
    fn default() -> Self {
        Self {
            company_data: CompanyUpdateData::default(),
            user_id_type: None,
        }
    }
}

/// 更新公司响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCompanyResponse {
    /// 公司信息
    pub company: Company,
}

impl ApiResponseTrait for UpdateCompanyResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 删除公司请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCompanyRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for DeleteCompanyRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 删除公司响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteCompanyResponse {}

impl ApiResponseTrait for DeleteCompanyResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取公司统计信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCompanyStatisticsRequest {
    /// 公司ID
    pub company_id: String,
}

/// 获取公司统计信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCompanyStatisticsResponse {
    /// 统计信息
    pub statistics: CompanyStatistics,
}

impl ApiResponseTrait for GetCompanyStatisticsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取公司组织架构请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrganizationStructureRequest {
    /// 公司ID
    pub company_id: String,
    /// 最大深度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i32>,
}

/// 获取公司组织架构响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrganizationStructureResponse {
    /// 组织架构信息
    pub structure: OrganizationStructure,
}

impl ApiResponseTrait for GetOrganizationStructureResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取子公司列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubsidiariesRequest {
    /// 母公司ID
    pub parent_company_id: String,
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

/// 获取子公司列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubsidiariesResponse {
    /// 子公司列表
    pub items: Vec<Company>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for GetSubsidiariesResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 公司搜索结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompanySearchResult {
    /// 公司ID
    pub company_id: String,
    /// 公司名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 统一社会信用代码
    pub unified_social_credit_code: Option<String>,
    /// 法定代表人
    pub legal_representative: Option<String>,
    /// 行业
    pub industry: Option<String>,
    /// 员工数量
    pub employee_count: i32,
    /// 匹配分数
    pub match_score: f64,
}

/// 公司统计信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompanyStatistics {
    /// 公司ID
    pub company_id: String,
    /// 总员工数
    pub total_employees: i32,
    /// 活跃员工数
    pub active_employees: i32,
    /// 非活跃员工数
    pub inactive_employees: i32,
    /// 总部门数
    pub total_departments: i32,
    /// 总职位数
    pub total_positions: i32,
    /// 活跃合同数
    pub active_contracts: i32,
    /// 近期入职人数
    pub recent_hires: i32,
    /// 近期离职人数
    pub recent_departures: i32,
    /// 平均工作年限
    pub avg_tenure_years: f64,
    /// 活跃率
    pub active_rate: f64,
    /// 性别分布
    pub gender_distribution: Option<serde_json::Value>,
    /// 年龄分布
    pub age_distribution: Option<serde_json::Value>,
    /// 部门分布
    pub department_distribution: Option<serde_json::Value>,
}

/// 组织架构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationStructure {
    /// 公司ID
    pub company_id: String,
    /// 根部门列表
    pub root_departments: Vec<DepartmentNode>,
    /// 总部门数
    pub total_departments: i32,
    /// 总员工数
    pub total_employees: i32,
    /// 更新时间
    pub updated_at: Option<String>,
}

/// 部门节点
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepartmentNode {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 负责人用户ID
    pub leader_user_id: Option<String>,
    /// 负责人姓名
    pub leader_name: Option<String>,
    /// 员工数量
    pub employee_count: i32,
    /// 子部门列表
    pub sub_departments: Vec<DepartmentNode>,
    /// 部门级别
    pub level: Option<i32>,
    /// 创建时间
    pub created_at: Option<String>,
}

/// 公司信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Company {
    /// 公司ID
    pub company_id: String,
    /// 公司名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 统一社会信用代码
    pub unified_social_credit_code: Option<String>,
    /// 法定代表人
    pub legal_representative: Option<String>,
    /// 注册资本
    pub registered_capital: Option<String>,
    /// 经营范围
    pub business_scope: Option<String>,
    /// 成立日期
    pub registration_date: Option<String>,
    /// 公司类型
    pub company_type: Option<String>,
    /// 行业
    pub industry: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 网站
    pub website: Option<String>,
    /// 员工数量
    pub employee_count: Option<i32>,
    /// 状态
    pub status: Option<String>,
    /// 母公司ID
    pub parent_company_id: Option<String>,
    /// 创建时间
    pub created_at: Option<String>,
    /// 更新时间
    pub updated_at: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

/// 公司创建数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompanyCreateData {
    /// 公司名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 统一社会信用代码
    pub unified_social_credit_code: Option<String>,
    /// 法定代表人
    pub legal_representative: Option<String>,
    /// 注册资本
    pub registered_capital: Option<String>,
    /// 经营范围
    pub business_scope: Option<String>,
    /// 成立日期
    pub registration_date: Option<String>,
    /// 公司类型
    pub company_type: Option<String>,
    /// 行业
    pub industry: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 网站
    pub website: Option<String>,
    /// 母公司ID
    pub parent_company_id: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

/// 公司更新数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompanyUpdateData {
    /// 公司名称
    pub name: Option<String>,
    /// 英文名称
    pub en_name: Option<String>,
    /// 统一社会信用代码
    pub unified_social_credit_code: Option<String>,
    /// 法定代表人
    pub legal_representative: Option<String>,
    /// 注册资本
    pub registered_capital: Option<String>,
    /// 经营范围
    pub business_scope: Option<String>,
    /// 成立时间
    pub registration_date: Option<String>,
    /// 公司类型
    pub company_type: Option<String>,
    /// 行业
    pub industry: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 网站
    pub website: Option<String>,
    /// 员工数量
    pub employee_count: Option<i32>,
    /// 状态
    pub status: Option<String>,
    /// 母公司ID
    pub parent_company_id: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}
