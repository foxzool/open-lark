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
        JobChange, JobChangeCreateRequest, JobChangeSearchRequest, Offboarding,
        OffboardingCreateRequest, OffboardingSearchRequest, PageResponse, PreHire,
        PreHireCreateRequest, PreHireSearchRequest,
    },
};

/// 员工生命周期管理服务（入职/离职/异动）
pub struct LifecycleService {
    pub config: Config,
}

/// 创建待入职响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PreHireCreateResponse {
    /// 待入职信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_hire: Option<PreHire>,
}

impl ApiResponseTrait for PreHireCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索待入职响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PreHireSearchResponse {
    /// 待入职信息列表
    #[serde(flatten)]
    pub pre_hires: PageResponse<PreHire>,
}

impl ApiResponseTrait for PreHireSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发起异动响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobChangeCreateResponse {
    /// 异动信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change: Option<JobChange>,
}

impl ApiResponseTrait for JobChangeCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索异动响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobChangeSearchResponse {
    /// 异动信息列表
    #[serde(flatten)]
    pub job_changes: PageResponse<JobChange>,
}

impl ApiResponseTrait for JobChangeSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发起离职响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OffboardingCreateResponse {
    /// 离职信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding: Option<Offboarding>,
}

impl ApiResponseTrait for OffboardingCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索离职响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OffboardingSearchResponse {
    /// 离职信息列表
    #[serde(flatten)]
    pub offboardings: PageResponse<Offboarding>,
}

impl ApiResponseTrait for OffboardingSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl LifecycleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 直接创建待入职
    ///
    /// 该接口用于直接创建待入职人员信息，适用于已确定招聘的候选人
    /// 创建入职流程，包含完整的个人信息和任职安排。
    ///
    /// # 参数
    ///
    /// - `request`: 创建待入职请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的待入职信息，包括：
    /// - 待入职ID和状态
    /// - 员工基本信息
    /// - 入职流程信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::{
    ///     PreHireCreateRequest, Person, Employment, I18nText
    /// };
    ///
    /// let request = PreHireCreateRequest {
    ///     person: Person {
    ///         name: Some(I18nText {
    ///             zh_cn: Some("张三".to_string()),
    ///             en_us: Some("Zhang San".to_string()),
    ///         }),
    ///         email: Some("zhangsan@example.com".to_string()),
    ///         phone: Some("13800138000".to_string()),
    ///         ..Default::default()
    ///     },
    ///     employment: Employment {
    ///         hire_date: Some("2024-03-01".to_string()),
    ///         employment_type: Some("full_time".to_string()),
    ///         ..Default::default()
    ///     },
    ///     job_datas: None,
    ///     onboarding_flow_id: Some("flow_123".to_string()),
    ///     expected_hire_date: Some("2024-03-01".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.lifecycle.create_pre_hire(request, None).await?;
    /// ```
    pub async fn create_pre_hire(
        &self,
        request: PreHireCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<PreHireCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_PRE_HIRES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 搜索待入职信息
    ///
    /// 该接口用于根据多种条件搜索待入职人员信息，支持按入职状态、
    /// 部门等条件进行筛选，并支持分页查询。
    ///
    /// # 参数
    ///
    /// - `request`: 搜索请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的待入职信息列表，包括：
    /// - 符合条件的待入职人员列表
    /// - 入职状态和进度信息
    /// - 分页信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::PreHireSearchRequest;
    ///
    /// let request = PreHireSearchRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    ///     employee_id_type: Some("employee_id".to_string()),
    ///     department_id_type: Some("open_department_id".to_string()),
    ///     onboarding_status: Some(vec!["pending".to_string(), "in_progress".to_string()]),
    ///     department_ids: Some(vec!["dept_123".to_string()]),
    /// };
    ///
    /// let response = client.corehr.lifecycle.search_pre_hire(request, None).await?;
    /// ```
    pub async fn search_pre_hire(
        &self,
        request: PreHireSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<PreHireSearchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_PRE_HIRES_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 发起员工异动
    ///
    /// 该接口用于发起员工异动流程，包括调岗、调薪、晋升等各种
    /// 人事变动，支持设置新的任职信息和生效日期。
    ///
    /// # 参数
    ///
    /// - `request`: 异动发起请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的异动信息，包括：
    /// - 异动ID和状态
    /// - 异动类型和原因
    /// - 原任职信息和新任职信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::{JobChangeCreateRequest, JobData};
    ///
    /// let request = JobChangeCreateRequest {
    ///     employee_id: "emp_123".to_string(),
    ///     job_change_type_id: "promotion".to_string(),
    ///     job_change_reason_id: Some("performance_excellent".to_string()),
    ///     effective_date: "2024-04-01".to_string(),
    ///     job_data: JobData {
    ///         job_id: Some("senior_engineer".to_string()),
    ///         job_level_id: Some("P7".to_string()),
    ///         department_id: Some("tech_dept".to_string()),
    ///         ..Default::default()
    ///     },
    ///     comments: Some("优秀员工晋升".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.lifecycle.create_job_change(request, None).await?;
    /// ```
    pub async fn create_job_change(
        &self,
        request: JobChangeCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobChangeCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_JOB_CHANGES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 搜索员工异动信息
    ///
    /// 该接口用于根据多种条件搜索员工异动信息，支持按异动状态、
    /// 异动类型、员工等条件进行筛选，并支持分页查询。
    ///
    /// # 参数
    ///
    /// - `request`: 搜索请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的异动信息列表，包括：
    /// - 符合条件的异动记录列表
    /// - 异动详细信息和状态
    /// - 分页信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::JobChangeSearchRequest;
    ///
    /// let request = JobChangeSearchRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    ///     employee_id_type: Some("employee_id".to_string()),
    ///     department_id_type: Some("open_department_id".to_string()),
    ///     employee_ids: Some(vec!["emp_123".to_string()]),
    ///     job_change_status: Some(vec!["approved".to_string(), "pending".to_string()]),
    ///     job_change_type_ids: Some(vec!["promotion".to_string(), "transfer".to_string()]),
    /// };
    ///
    /// let response = client.corehr.lifecycle.search_job_change(request, None).await?;
    /// ```
    pub async fn search_job_change(
        &self,
        request: JobChangeSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobChangeSearchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_JOB_CHANGES_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 操作员工离职
    ///
    /// 该接口用于发起员工离职流程，包括主动离职、辞退等各种
    /// 离职类型，支持设置离职原因和离职日期。
    ///
    /// # 参数
    ///
    /// - `request`: 离职发起请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的离职信息，包括：
    /// - 离职ID和状态
    /// - 离职原因和类型
    /// - 离职日期
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::OffboardingCreateRequest;
    ///
    /// let request = OffboardingCreateRequest {
    ///     employee_id: "emp_123".to_string(),
    ///     offboarding_reason_id: "personal_reason".to_string(),
    ///     offboarding_date: "2024-05-31".to_string(),
    ///     offboarding_type: Some("voluntary".to_string()),
    ///     comments: Some("个人发展需要".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.lifecycle.create_offboarding(request, None).await?;
    /// ```
    pub async fn create_offboarding(
        &self,
        request: OffboardingCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OffboardingCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_OFFBOARDINGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 搜索离职信息
    ///
    /// 该接口用于根据多种条件搜索员工离职信息，支持按离职状态、
    /// 离职原因、员工等条件进行筛选，并支持分页查询。
    ///
    /// # 参数
    ///
    /// - `request`: 搜索请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的离职信息列表，包括：
    /// - 符合条件的离职记录列表
    /// - 离职详细信息和状态
    /// - 分页信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::OffboardingSearchRequest;
    ///
    /// let request = OffboardingSearchRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    ///     employee_id_type: Some("employee_id".to_string()),
    ///     department_id_type: Some("open_department_id".to_string()),
    ///     employee_ids: Some(vec!["emp_123".to_string()]),
    ///     offboarding_status: Some(vec!["approved".to_string(), "pending".to_string()]),
    ///     offboarding_reason_ids: Some(vec!["personal_reason".to_string()]),
    /// };
    ///
    /// let response = client.corehr.lifecycle.search_offboarding(request, None).await?;
    /// ```
    pub async fn search_offboarding(
        &self,
        request: OffboardingSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OffboardingSearchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_OFFBOARDINGS_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }
}
