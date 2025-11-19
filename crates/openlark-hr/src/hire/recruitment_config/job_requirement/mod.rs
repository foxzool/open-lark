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

use crate::hire::models::{
    CommonResponse, JobRequirement, JobRequirementCreateRequest, PageResponse
};

/// 招聘需求服务
pub struct JobRequirementService {
    pub config: Config,
}

/// 招聘需求列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobRequirementListResponse {
    /// 招聘需求列表
    #[serde(flatten)]
    pub requirements: PageResponse<JobRequirement>,
}

impl ApiResponseTrait for JobRequirementListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 招聘需求详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobRequirementDetailResponse {
    /// 招聘需求信息
    pub requirement: JobRequirement,
}

impl ApiResponseTrait for JobRequirementDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 招聘需求操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobRequirementOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 需求ID
    pub requirement_id: Option<String>,
}

impl ApiResponseTrait for JobRequirementOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 招聘需求列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobRequirementListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 职位ID
    pub job_id: Option<String>,
    /// 需求状态
    pub status: Option<String>,
}

impl JobRequirementService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建招聘需求
    ///
    /// 该接口用于为指定职位创建招聘需求，设置招聘人数、
    /// 期望入职时间等具体要求。创建成功后可用于跟踪
    /// 招聘进度和统计数据。
    ///
    /// # 参数
    ///
    /// - `request`: 招聘需求创建请求参数，包括：
    ///   - `name`: 需求名称（必填）
    ///   - `job_id`: 关联职位ID（必填）
    ///   - `headcount`: 需求人数（必填）
    ///   - `description`: 需求描述
    ///   - `expected_entry_time`: 期望入职时间
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回招聘需求创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `requirement_id`: 创建的需求ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::JobRequirementCreateRequest;
    ///
    /// let request = JobRequirementCreateRequest {
    ///     name: "急招Java高级工程师".to_string(),
    ///     job_id: "job_123456".to_string(),
    ///     headcount: 3,
    ///     description: Some("项目紧急需求，要求有微服务架构经验".to_string()),
    ///     expected_entry_time: Some("2024-02-01".to_string()),
    /// };
    ///
    /// let response = client.hire.recruitment_config.job_requirement.create_requirement(request, None).await?;
    /// ```
    pub async fn create_requirement(
        &self,
        request: JobRequirementCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobRequirementOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_JOB_REQUIREMENTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取招聘需求详情
    ///
    /// 该接口用于获取指定招聘需求的详细信息，包括需求
    /// 基本信息、关联职位、招聘进度等完整数据。
    ///
    /// # 参数
    ///
    /// - `requirement_id`: 招聘需求ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回招聘需求详细信息，包括：
    /// - 需求基本信息（名称、描述、人数等）
    /// - 关联职位信息
    /// - 需求状态和进度
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let requirement_id = "req_123456";
    /// let response = client.hire.recruitment_config.job_requirement.get_requirement_detail(requirement_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("需求名称: {}", data.requirement.name);
    ///     println!("需求人数: {}", data.requirement.headcount);
    ///     println!("关联职位: {}", data.requirement.job_id);
    /// }
    /// ```
    pub async fn get_requirement_detail(
        &self,
        requirement_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobRequirementDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_JOB_REQUIREMENT_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取招聘需求列表
    ///
    /// 该接口用于获取企业的招聘需求列表，支持按职位、
    /// 状态等条件筛选。返回的列表包含需求基本信息，
    /// 可用于需求管理和进度跟踪。
    ///
    /// # 参数
    ///
    /// - `request`: 招聘需求列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `job_id`: 职位ID筛选
    ///   - `status`: 需求状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的招聘需求列表，包括：
    /// - 需求基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::job_requirement::JobRequirementListRequest;
    ///
    /// let request = JobRequirementListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     job_id: Some("job_123456".to_string()),
    ///     status: Some("active".to_string()),
    /// };
    ///
    /// let response = client.hire.recruitment_config.job_requirement.list_requirements(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("需求总数: {}", data.requirements.items.len());
    ///     for requirement in &data.requirements.items {
    ///         println!("需求: {} (人数: {})", requirement.name, requirement.headcount);
    ///     }
    /// }
    /// ```
    pub async fn list_requirements(
        &self,
        request: JobRequirementListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobRequirementListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_JOB_REQUIREMENTS.to_string());
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

        if let Some(job_id) = request.job_id {
            api_req.query_params.insert("job_id", job_id);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新招聘需求
    ///
    /// 该接口用于更新现有招聘需求的信息，支持修改需求
    /// 人数、期望入职时间、需求描述等配置。
    ///
    /// # 参数
    ///
    /// - `requirement_id`: 招聘需求ID
    /// - `request`: 招聘需求更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::JobRequirementCreateRequest;
    ///
    /// let requirement_id = "req_123456";
    /// let request = JobRequirementCreateRequest {
    ///     name: "急招Java架构师".to_string(),
    ///     job_id: "job_123456".to_string(),
    ///     headcount: 2,
    ///     description: Some("项目技术难度较高，需要有分布式系统设计经验".to_string()),
    ///     expected_entry_time: Some("2024-01-15".to_string()),
    /// };
    ///
    /// let response = client.hire.recruitment_config.job_requirement.update_requirement(requirement_id, request, None).await?;
    /// ```
    pub async fn update_requirement(
        &self,
        requirement_id: &str,
        request: JobRequirementCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobRequirementOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_JOB_REQUIREMENT_UPDATE,
            "job_requirement_id",
            requirement_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 删除招聘需求
    ///
    /// 该接口用于删除指定的招聘需求。删除后的需求
    /// 将不再显示在需求列表中。
    ///
    /// # 参数
    ///
    /// - `requirement_id`: 招聘需求ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let requirement_id = "req_123456";
    /// let response = client.hire.recruitment_config.job_requirement.delete_requirement(requirement_id, None).await?;
    /// ```
    pub async fn delete_requirement(
        &self,
        requirement_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobRequirementOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_JOB_REQUIREMENT_DELETE,
            "job_requirement_id",
            requirement_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}
