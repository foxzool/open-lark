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
    CommonResponse, Job, JobCreateRequest, JobListRequest, JobUpdateRequest, PageResponse,
};

/// 职位服务
pub struct JobService {
    pub config: Config,
}

/// 职位列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobListResponse {
    /// 职位列表
    #[serde(flatten)]
    pub jobs: PageResponse<Job>,
}

impl ApiResponseTrait for JobListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 职位详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobDetailResponse {
    /// 职位信息
    pub job: Job,
}

impl ApiResponseTrait for JobDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 职位操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 职位ID
    pub job_id: Option<String>,
}

impl ApiResponseTrait for JobOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl JobService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新建职位
    ///
    /// 该接口用于创建新的招聘职位，支持设置职位基本信息、
    /// 工作地点、招聘人员、面试官等详细配置。创建成功后
    /// 职位将进入待发布状态，可通过其他接口进行发布操作。
    ///
    /// # 参数
    ///
    /// - `request`: 职位创建请求参数，包括：
    ///   - `title`: 职位名称（必填）
    ///   - `description`: 职位描述
    ///   - `requirement`: 职位要求
    ///   - `job_type`: 职位类型
    ///   - `location_ids`: 工作地点ID列表
    ///   - `recruiter_ids`: 招聘人员ID列表
    ///   - `interviewer_ids`: 面试官ID列表
    ///   - `custom_fields`: 自定义字段
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回职位创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `job_id`: 创建的职位ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::JobCreateRequest;
    ///
    /// let request = JobCreateRequest {
    ///     title: "高级软件工程师".to_string(),
    ///     description: Some("负责后端系统开发和维护".to_string()),
    ///     requirement: Some("3年以上Java开发经验".to_string()),
    ///     job_type: Some("full_time".to_string()),
    ///     location_ids: vec!["loc_123".to_string()],
    ///     recruiter_ids: vec!["user_456".to_string()],
    ///     interviewer_ids: vec!["user_789".to_string()],
    ///     custom_fields: None,
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.job.create_job(request, None).await?;
    /// ```
    pub async fn create_job(
        &self,
        request: JobCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_JOB_COMBINED_CREATE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 更新职位
    ///
    /// 该接口用于更新现有职位的基本信息，支持修改职位名称、
    /// 描述、要求、工作地点、招聘人员等配置。更新后的职位
    /// 信息将立即生效。
    ///
    /// # 参数
    ///
    /// - `request`: 职位更新请求参数，包括：
    ///   - `job_id`: 职位ID（必填）
    ///   - `title`: 职位名称
    ///   - `description`: 职位描述
    ///   - `requirement`: 职位要求
    ///   - `location_ids`: 工作地点ID列表
    ///   - `recruiter_ids`: 招聘人员ID列表
    ///   - `interviewer_ids`: 面试官ID列表
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::JobUpdateRequest;
    ///
    /// let request = JobUpdateRequest {
    ///     job_id: "job_123456".to_string(),
    ///     title: Some("资深软件工程师".to_string()),
    ///     description: Some("负责核心业务系统开发和架构设计".to_string()),
    ///     requirement: Some("5年以上Java开发经验，有大型项目架构经验".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.job.update_job(request, None).await?;
    /// ```
    pub async fn update_job(
        &self,
        request: JobUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_JOB_COMBINED_UPDATE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取职位详情
    ///
    /// 该接口用于获取指定职位的详细信息，包括职位基本信息、
    /// 工作地点、招聘人员、面试官、自定义字段等完整数据。
    ///
    /// # 参数
    ///
    /// - `job_id`: 职位ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回职位详细信息，包括：
    /// - 职位基本信息（名称、描述、要求等）
    /// - 职位状态和类型
    /// - 工作地点列表
    /// - 招聘人员和面试官信息
    /// - 自定义字段数据
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let job_id = "job_123456";
    /// let response = client.hire.recruitment_config.job.get_job_detail(job_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("职位名称: {}", data.job.title);
    ///     println!("职位状态: {}", data.job.status);
    ///     println!("工作地点数: {}", data.job.locations.len());
    /// }
    /// ```
    pub async fn get_job_detail(
        &self,
        job_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_JOB_GET_DETAIL, "job_id", job_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取职位列表
    ///
    /// 该接口用于获取企业的职位列表，支持按状态、部门、
    /// 职位类型、创建时间等条件筛选。返回的列表包含职位
    /// 基本信息，可用于职位浏览和管理。
    ///
    /// # 参数
    ///
    /// - `request`: 职位列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `status`: 职位状态筛选
    ///   - `department_id`: 部门ID筛选
    ///   - `job_type`: 职位类型筛选
    ///   - `created_start_time`: 创建时间开始
    ///   - `created_end_time`: 创建时间结束
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的职位列表，包括：
    /// - 职位基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::JobListRequest;
    ///
    /// let request = JobListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    ///     department_id: Some("dept_123".to_string()),
    ///     job_type: Some("full_time".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.job.list_jobs(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("职位总数: {}", data.jobs.items.len());
    ///     for job in &data.jobs.items {
    ///         println!("职位: {} ({})", job.title, job.status);
    ///     }
    /// }
    /// ```
    pub async fn list_jobs(
        &self,
        request: JobListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_JOBS.to_string());
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

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(department_id) = request.department_id {
            api_req.query_params.insert("department_id", department_id);
        }

        if let Some(job_type) = request.job_type {
            api_req.query_params.insert("job_type", job_type);
        }

        if let Some(created_start_time) = request.created_start_time {
            api_req
                .query_params
                .insert("created_start_time", created_start_time);
        }

        if let Some(created_end_time) = request.created_end_time {
            api_req
                .query_params
                .insert("created_end_time", created_end_time);
        }

        Transport::request(api_req, &self.config, option).await
    }
    /// 关闭职位
    ///
    /// 该接口用于关闭指定的招聘职位。关闭后的职位将停止
    /// 接收新的投递，但不影响已有的投递流程。
    ///
    /// # 参数
    ///
    /// - `job_id`: 职位ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let job_id = "job_123456";
    /// let response = client.hire.recruitment_config.job.close_job(job_id, None).await?;
    /// ```
    pub async fn close_job(
        &self,
        job_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_JOB_CLOSE, "job_id", job_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 重启职位
    ///
    /// 该接口用于重新开启已关闭的招聘职位。重启后的职位
    /// 将恢复接收新的投递。
    ///
    /// # 参数
    ///
    /// - `job_id`: 职位ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let job_id = "job_123456";
    /// let response = client.hire.recruitment_config.job.open_job(job_id, None).await?;
    /// ```
    pub async fn open_job(
        &self,
        job_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_JOB_OPEN, "job_id", job_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}
