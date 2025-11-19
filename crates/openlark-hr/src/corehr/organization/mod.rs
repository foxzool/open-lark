use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::corehr::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::corehr::models::{
        Company, CompanyCreateRequest, Department, DepartmentCreateRequest, PageResponse,
    },
};

/// 组织管理服务
pub struct OrganizationService {
    pub config: Config,
}

/// 创建部门响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentCreateResponse {
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Department>,
}

impl ApiResponseTrait for DepartmentCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询部门响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentBatchGetResponse {
    /// 部门信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Department>>,
}

impl ApiResponseTrait for DepartmentBatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 部门架构树响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentTreeResponse {
    /// 部门树结构
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Department>>,
}

impl ApiResponseTrait for DepartmentTreeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建公司响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyCreateResponse {
    /// 公司信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,
}

impl ApiResponseTrait for CompanyCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询公司响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyListResponse {
    /// 公司信息列表
    #[serde(flatten)]
    pub companies: PageResponse<Company>,
}

impl ApiResponseTrait for CompanyListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl OrganizationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建部门
    ///
    /// 该接口用于创建新的部门，支持设置部门名称、上级部门、
    /// 部门负责人等信息，并支持自定义字段。
    ///
    /// # 参数
    ///
    /// - `request`: 创建部门请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的部门信息，包括：
    /// - 部门ID和基本信息
    /// - 部门层级关系
    /// - 生效时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::{DepartmentCreateRequest, I18nText};
    ///
    /// let request = DepartmentCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("技术部".to_string()),
    ///         en_us: Some("Technology Department".to_string()),
    ///     },
    ///     parent_department_id: Some("parent_dept_123".to_string()),
    ///     manager: Some("emp_manager_456".to_string()),
    ///     code: Some("TECH".to_string()),
    ///     description: Some(I18nText {
    ///         zh_cn: Some("负责技术研发工作".to_string()),
    ///         en_us: Some("Responsible for technology R&D".to_string()),
    ///     }),
    ///     effective_time: Some("2024-01-01".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.organization.create_department(request, None).await?;
    /// ```
    pub async fn create_department(
        &self,
        request: DepartmentCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<DepartmentCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_DEPARTMENTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量查询部门
    ///
    /// 该接口用于通过部门ID列表批量获取部门信息，支持获取
    /// 部门的基本信息、层级关系等。
    ///
    /// # 参数
    ///
    /// - `department_ids`: 部门ID列表
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回部门信息列表，包括：
    /// - 部门基本信息
    /// - 层级关系
    /// - 自定义字段
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let department_ids = vec!["dept_123".to_string(), "dept_456".to_string()];
    /// let response = client.corehr.organization.batch_get_departments(department_ids, None).await?;
    /// ```
    pub async fn batch_get_departments(
        &self,
        department_ids: Vec<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<DepartmentBatchGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_DEPARTMENTS_BATCH_GET.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&serde_json::json!({
                "department_ids": department_ids
            }))
            .unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询部门架构树
    ///
    /// 该接口用于查询指定生效日期的部门架构树，返回完整的
    /// 部门层级结构，便于展示组织架构。
    ///
    /// # 参数
    ///
    /// - `effective_date`: 生效日期（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回部门树结构，包括：
    /// - 完整的部门层级关系
    /// - 各级部门的基本信息
    /// - 子部门列表
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 查询当前生效的部门架构树
    /// let response = client.corehr.organization.get_department_tree(None, None).await?;
    ///
    /// // 查询指定日期的部门架构树
    /// let response = client.corehr.organization.get_department_tree(
    ///     Some("2024-01-01".to_string()),
    ///     None
    /// ).await?;
    /// ```
    pub async fn get_department_tree(
        &self,
        effective_date: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<DepartmentTreeResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_DEPARTMENTS_TREE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(date) = effective_date {
            api_req.query_params.insert("effective_date", date);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 创建公司
    ///
    /// 该接口用于创建新的公司信息，支持设置公司名称、类型、
    /// 法定名称等信息，并支持自定义字段。
    ///
    /// # 参数
    ///
    /// - `request`: 创建公司请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的公司信息，包括：
    /// - 公司ID和基本信息
    /// - 公司类型和法定名称
    /// - 生效时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::{CompanyCreateRequest, I18nText};
    ///
    /// let request = CompanyCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("北京科技有限公司".to_string()),
    ///         en_us: Some("Beijing Technology Co., Ltd.".to_string()),
    ///     },
    ///     company_type: Some("subsidiary".to_string()),
    ///     legal_name: Some(I18nText {
    ///         zh_cn: Some("北京科技有限公司".to_string()),
    ///         en_us: Some("Beijing Technology Co., Ltd.".to_string()),
    ///     }),
    ///     code: Some("BJ_TECH".to_string()),
    ///     effective_time: Some("2024-01-01".to_string()),
    ///     primary_location_id: Some("location_123".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.organization.create_company(request, None).await?;
    /// ```
    pub async fn create_company(
        &self,
        request: CompanyCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CompanyCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_COMPANIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量查询公司
    ///
    /// 该接口用于分页查询公司信息列表，支持获取所有公司的
    /// 基本信息和状态。
    ///
    /// # 参数
    ///
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的公司信息列表，包括：
    /// - 公司基本信息
    /// - 公司状态和类型
    /// - 分页信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 查询第一页公司信息
    /// let response = client.corehr.organization.list_companies(Some(50), None, None).await?;
    ///
    /// // 查询下一页
    /// let response = client.corehr.organization.list_companies(
    ///     Some(50),
    ///     Some("next_page_token".to_string()),
    ///     None
    /// ).await?;
    /// ```
    pub async fn list_companies(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<CompanyListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: COREHR_COMPANIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(size) = page_size {
            api_req.query_params.insert("page_size", size.to_string());
        }

        if let Some(token) = page_token {
            api_req.query_params.insert("page_token", token);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
