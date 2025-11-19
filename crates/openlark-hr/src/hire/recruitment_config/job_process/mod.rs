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

use crate::hire::models::{CommonResponse, I18nText, PageResponse, UserId};

/// 招聘流程服务
pub struct JobProcessService {
    pub config: Config,
}

/// 招聘阶段信息
#[derive(Debug, Serialize, Deserialize)]
pub struct JobProcessStage {
    /// 阶段ID
    pub id: String,
    /// 阶段名称
    pub name: I18nText,
    /// 阶段类型
    pub stage_type: String,
    /// 阶段顺序
    pub order: u32,
    /// 是否必需阶段
    pub is_required: bool,
    /// 阶段配置
    pub config: Option<StageConfig>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 阶段配置信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StageConfig {
    // TODO: Add fields
}

/// 通知设置
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSettings {
    /// 是否启用通知
    pub enabled: bool,
    /// 通知接收人列表
    pub recipients: Vec<UserId>,
    /// 通知模板ID
    pub template_id: Option<String>,
}

/// 招聘流程信息
#[derive(Debug, Serialize, Deserialize)]
pub struct JobProcess {
    /// 流程ID
    pub id: String,
    /// 流程名称
    pub name: I18nText,
    /// 流程描述
    pub description: Option<I18nText>,
    /// 流程类型
    pub process_type: String,
    /// 是否默认流程
    pub is_default: bool,
    /// 流程状态
    pub status: String,
    /// 阶段列表
    pub stages: Vec<JobProcessStage>,
    /// 适用职位类型
    pub applicable_job_types: Vec<String>,
    /// 创建人
    pub creator: Option<UserId>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 招聘流程创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobProcessCreateRequest {
    /// 流程名称
    pub name: I18nText,
    /// 流程描述
    pub description: Option<I18nText>,
    /// 流程类型
    pub process_type: String,
    /// 是否默认流程
    pub is_default: Option<bool>,
    /// 阶段配置列表
    pub stages: Vec<JobProcessStageCreateRequest>,
    /// 适用职位类型
    pub applicable_job_types: Vec<String>,
}

/// 招聘流程阶段创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobProcessStageCreateRequest {
    /// 阶段名称
    pub name: I18nText,
    /// 阶段类型
    pub stage_type: String,
    /// 阶段顺序
    pub order: u32,
    /// 是否必需阶段
    pub is_required: Option<bool>,
    /// 阶段配置
    pub config: Option<StageConfig>,
}

/// 招聘流程列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobProcessListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 流程类型
    pub process_type: Option<String>,
    /// 流程状态
    pub status: Option<String>,
}

/// 招聘流程列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobProcessListResponse {
    /// 招聘流程列表
    #[serde(flatten)]
    pub processes: PageResponse<JobProcess>,
}

impl ApiResponseTrait for JobProcessListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 招聘流程详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobProcessDetailResponse {
    /// 招聘流程信息
    pub process: JobProcess,
}

impl ApiResponseTrait for JobProcessDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 招聘流程操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JobProcessOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 流程ID
    pub process_id: Option<String>,
}

impl ApiResponseTrait for JobProcessOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl JobProcessService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建招聘流程
    ///
    /// 该接口用于创建新的招聘流程模板，定义招聘的各个
    /// 阶段、流程配置和适用范围。创建的流程可应用于
    /// 不同的职位招聘中。
    ///
    /// # 参数
    ///
    /// - `request`: 招聘流程创建请求参数，包括：
    ///   - `name`: 流程名称（必填）
    ///   - `process_type`: 流程类型（必填）
    ///   - `stages`: 阶段配置列表（必填）
    ///   - `description`: 流程描述
    ///   - `is_default`: 是否默认流程
    ///   - `applicable_job_types`: 适用职位类型
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回招聘流程创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `process_id`: 创建的流程ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::job_process::{
    ///     JobProcessCreateRequest, JobProcessStageCreateRequest
    /// };
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let request = JobProcessCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("技术岗位招聘流程".to_string()),
    ///         en_us: Some("Technical Position Recruitment Process".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     process_type: "technical".to_string(),
    ///     stages: vec![
    ///         JobProcessStageCreateRequest {
    ///             name: I18nText {
    ///                 zh_cn: Some("简历筛选".to_string()),
    ///                 en_us: Some("Resume Screening".to_string()),
    ///                 ja_jp: None,
    ///             },
    ///             stage_type: "resume_review".to_string(),
    ///             order: 1,
    ///             is_required: Some(true),
    ///             config: None,
    ///         },
    ///     ],
    ///     applicable_job_types: vec!["software_engineer".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.job_process.create_process(request, None).await?;
    /// ```
    pub async fn create_process(
        &self,
        request: JobProcessCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobProcessOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_JOB_PROCESSES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取招聘流程详情
    ///
    /// 该接口用于获取指定招聘流程的详细信息，包括流程
    /// 基本信息、阶段配置、适用范围等完整数据。
    ///
    /// # 参数
    ///
    /// - `process_id`: 招聘流程ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回招聘流程详细信息，包括：
    /// - 流程基本信息（名称、类型、状态等）
    /// - 阶段配置列表
    /// - 适用职位类型
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let process_id = "process_123456";
    /// let response = client.hire.recruitment_config.job_process.get_process_detail(process_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("流程名称: {:?}", data.process.name.zh_cn);
    ///     println!("流程类型: {}", data.process.process_type);
    ///     println!("阶段数量: {}", data.process.stages.len());
    /// }
    /// ```
    pub async fn get_process_detail(
        &self,
        process_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobProcessDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_JOB_PROCESS_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取招聘流程列表
    ///
    /// 该接口用于获取企业的招聘流程列表，支持按类型、
    /// 状态等条件筛选。返回的列表包含流程基本信息，
    /// 可用于流程管理和选择。
    ///
    /// # 参数
    ///
    /// - `request`: 招聘流程列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `process_type`: 流程类型筛选
    ///   - `status`: 流程状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的招聘流程列表，包括：
    /// - 流程基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::job_process::JobProcessListRequest;
    ///
    /// let request = JobProcessListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     process_type: Some("technical".to_string()),
    ///     status: Some("active".to_string()),
    /// };
    ///
    /// let response = client.hire.recruitment_config.job_process.list_processes(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("流程总数: {}", data.processes.items.len());
    ///     for process in &data.processes.items {
    ///         println!("流程: {:?} (类型: {})", process.name.zh_cn, process.process_type);
    ///     }
    /// }
    /// ```
    pub async fn list_processes(
        &self,
        request: JobProcessListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobProcessListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_JOB_PROCESSES.to_string());
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

        if let Some(process_type) = request.process_type {
            api_req.query_params.insert("process_type", process_type);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新招聘流程
    ///
    /// 该接口用于更新现有招聘流程的配置，支持修改流程
    /// 名称、阶段配置、适用范围等信息。
    ///
    /// # 参数
    ///
    /// - `process_id`: 招聘流程ID
    /// - `request`: 招聘流程更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::job_process::JobProcessCreateRequest;
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let process_id = "process_123456";
    /// let request = JobProcessCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("高级技术岗位招聘流程".to_string()),
    ///         en_us: Some("Senior Technical Position Recruitment Process".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     process_type: "senior_technical".to_string(),
    ///     stages: vec![],
    ///     applicable_job_types: vec!["senior_engineer".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.job_process.update_process(process_id, request, None).await?;
    /// ```
    pub async fn update_process(
        &self,
        process_id: &str,
        request: JobProcessCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobProcessOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_JOB_PROCESS_UPDATE,
            "job_process_id",
            process_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 删除招聘流程
    ///
    /// 该接口用于删除指定的招聘流程。删除后的流程
    /// 将不再可用于新的职位招聘。
    ///
    /// # 参数
    ///
    /// - `process_id`: 招聘流程ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let process_id = "process_123456";
    /// let response = client.hire.recruitment_config.job_process.delete_process(process_id, None).await?;
    /// ```
    pub async fn delete_process(
        &self,
        process_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<JobProcessOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_JOB_PROCESS_DELETE,
            "job_process_id",
            process_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}
