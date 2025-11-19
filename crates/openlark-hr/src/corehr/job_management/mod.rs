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
        Job, JobCreateRequest, JobFamily, JobFamilyCreateRequest, JobGrade, JobGradeCreateRequest,
        JobLevel, JobLevelCreateRequest, PageResponse,
    },
};

/// 岗职务管理服务
pub struct JobManagementService {
    pub config: Config,
}

/// 创建序列响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobFamilyCreateResponse {
    /// 序列信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family: Option<JobFamily>,
}

impl ApiResponseTrait for JobFamilyCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询序列响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobFamilyListResponse {
    /// 序列信息列表
    #[serde(flatten)]
    pub job_families: PageResponse<JobFamily>,
}

impl ApiResponseTrait for JobFamilyListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建职级响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobLevelCreateResponse {
    /// 职级信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level: Option<JobLevel>,
}

impl ApiResponseTrait for JobLevelCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询职级响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobLevelListResponse {
    /// 职级信息列表
    #[serde(flatten)]
    pub job_levels: PageResponse<JobLevel>,
}

impl ApiResponseTrait for JobLevelListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建职等响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobGradeCreateResponse {
    /// 职等信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_grade: Option<JobGrade>,
}

impl ApiResponseTrait for JobGradeCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询职等响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobGradeQueryResponse {
    /// 职等信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<JobGrade>>,
}

impl ApiResponseTrait for JobGradeQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建职务响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobCreateResponse {
    /// 职务信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

impl ApiResponseTrait for JobCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询职务响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobListResponse {
    /// 职务信息列表
    #[serde(flatten)]
    pub jobs: PageResponse<Job>,
}

impl ApiResponseTrait for JobListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl JobManagementService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建序列
    ///
    /// 该接口用于创建新的序列信息，序列是对相同或相似岗位的分类，
    /// 帮助企业更好地管理和规划职业发展路径。
    ///
    /// # 参数
    ///
    /// - `request`: 创建序列请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的序列信息，包括：
    /// - 序列ID和基本信息
    /// - 序列名称和编码
    /// - 生效时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::{JobFamilyCreateRequest, I18nText};
    ///
    /// let request = JobFamilyCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("技术序列".to_string()),
    ///         en_us: Some("Technology".to_string()),
    ///     },
    ///     code: Some("TECH".to_string()),
    ///     effective_time: Some("2024-01-01".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.job_management.create_job_family(request, None).await?;
    /// ```
    pub async fn create_job_family(
        &self,
        request: JobFamilyCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobFamilyCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_JOB_FAMILIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量查询序列
    ///
    /// 该接口用于分页查询序列信息列表，支持获取所有序列的
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
    /// 返回分页的序列信息列表，包括：
    /// - 序列基本信息
    /// - 序列状态和编码
    /// - 分页信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 查询第一页序列信息
    /// let response = client.corehr.job_management.list_job_families(Some(50), None, None).await?;
    /// ```
    pub async fn list_job_families(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobFamilyListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: COREHR_JOB_FAMILIES.to_string(),
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

    /// 创建职级
    ///
    /// 该接口用于创建新的职级信息，职级用于定义员工在组织中的
    /// 层级位置和发展阶梯。
    ///
    /// # 参数
    ///
    /// - `request`: 创建职级请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的职级信息，包括：
    /// - 职级ID和基本信息
    /// - 职级名称、编码和描述
    /// - 职级顺序
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::{JobLevelCreateRequest, I18nText};
    ///
    /// let request = JobLevelCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("高级工程师".to_string()),
    ///         en_us: Some("Senior Engineer".to_string()),
    ///     },
    ///     code: Some("P6".to_string()),
    ///     description: Some(I18nText {
    ///         zh_cn: Some("资深技术人员".to_string()),
    ///         en_us: Some("Senior technical staff".to_string()),
    ///     }),
    ///     order: Some(6),
    ///     effective_time: Some("2024-01-01".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.job_management.create_job_level(request, None).await?;
    /// ```
    pub async fn create_job_level(
        &self,
        request: JobLevelCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobLevelCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_JOB_LEVELS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量查询职级
    ///
    /// 该接口用于分页查询职级信息列表，支持获取所有职级的
    /// 基本信息和排序。
    ///
    /// # 参数
    ///
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的职级信息列表，包括：
    /// - 职级基本信息
    /// - 职级顺序和状态
    /// - 分页信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 查询第一页职级信息
    /// let response = client.corehr.job_management.list_job_levels(Some(50), None, None).await?;
    /// ```
    pub async fn list_job_levels(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobLevelListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: COREHR_JOB_LEVELS.to_string(),
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

    /// 创建职等
    ///
    /// 该接口用于创建新的职等信息，职等是对同一职级下不同
    /// 等级的细分，用于更精细的职业发展管理。
    ///
    /// # 参数
    ///
    /// - `request`: 创建职等请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的职等信息，包括：
    /// - 职等ID和基本信息
    /// - 职等名称、编码和描述
    /// - 所属序列信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::{JobGradeCreateRequest, I18nText};
    ///
    /// let request = JobGradeCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("高级工程师A".to_string()),
    ///         en_us: Some("Senior Engineer A".to_string()),
    ///     },
    ///     code: Some("P6A".to_string()),
    ///     description: Some(I18nText {
    ///         zh_cn: Some("高级工程师A等级".to_string()),
    ///         en_us: Some("Senior Engineer Grade A".to_string()),
    ///     }),
    ///     job_family_id: Some("tech_family_123".to_string()),
    ///     effective_time: Some("2024-01-01".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.job_management.create_job_grade(request, None).await?;
    /// ```
    pub async fn create_job_grade(
        &self,
        request: JobGradeCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobGradeCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_JOB_GRADES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }
    /// 查询职等
    ///
    /// 该接口用于查询职等信息，支持按多种条件筛选职等。
    ///
    /// # 参数
    ///
    /// - `job_family_id`: 序列ID（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回职等信息列表，包括：
    /// - 职等基本信息
    /// - 所属序列信息
    /// - 职等状态
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 查询特定序列的职等
    /// let response = client.corehr.job_management.query_job_grades(
    ///     Some("tech_family_123".to_string()),
    ///     None
    /// ).await?;
    /// ```
    pub async fn query_job_grades(
        &self,
        job_family_id: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobGradeQueryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_JOB_GRADES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(family_id) = job_family_id {
            api_req.query_params.insert("job_family_id", family_id);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 创建职务
    ///
    /// 该接口用于创建新的职务信息，职务是具体的工作岗位，
    /// 定义了员工的具体工作职责和要求。
    ///
    /// # 参数
    ///
    /// - `request`: 创建职务请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的职务信息，包括：
    /// - 职务ID和基本信息
    /// - 职务名称、编码和描述
    /// - 所属序列信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::{JobCreateRequest, I18nText};
    ///
    /// let request = JobCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("前端开发工程师".to_string()),
    ///         en_us: Some("Frontend Engineer".to_string()),
    ///     },
    ///     code: Some("FE".to_string()),
    ///     description: Some(I18nText {
    ///         zh_cn: Some("负责前端开发工作".to_string()),
    ///         en_us: Some("Responsible for frontend development".to_string()),
    ///     }),
    ///     job_family_id: Some("tech_family_123".to_string()),
    ///     effective_time: Some("2024-01-01".to_string()),
    ///     custom_fields: None,
    /// };
    ///
    /// let response = client.corehr.job_management.create_job(request, None).await?;
    /// ```
    pub async fn create_job(
        &self,
        request: JobCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_JOBS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置
        Transport::request(api_req, &self.config, option).await
    }

    /// 批量查询职务
    ///
    /// 该接口用于分页查询职务信息列表，支持获取所有职务的
    /// 基本信息和所属序列。
    ///
    /// # 参数
    ///
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的职务信息列表，包括：
    /// - 职务基本信息
    /// - 所属序列信息
    /// - 分页信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 查询第一页职务信息
    /// let response = client.corehr.job_management.list_jobs(Some(50), None, None).await?;
    /// ```
    pub async fn list_jobs(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: COREHR_JOBS.to_string(),
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
