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

use crate::hire::models::{CommonResponse, PageResponse};

/// 外部系统服务
pub struct ExternalSystemService {
    pub config: Config,
}

/// 外部系统配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalSystemConfig {
    // TODO: Add fields
}

/// 外部系统同步记录
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemSyncRecord {
    /// 同步记录ID
    pub id: String,
    /// 外部系统配置ID
    pub system_config_id: String,
    /// 同步类型
    pub sync_type: String,
    /// 同步状态
    pub status: String,
    /// 同步开始时间
    pub start_time: String,
    /// 同步结束时间
    pub end_time: Option<String>,
    /// 同步结果统计
    pub sync_statistics: Option<SyncStatistics>,
    /// 错误信息
    pub error_message: Option<String>,
    /// 同步日志
    pub sync_logs: Vec<String>,
    /// 创建时间
    pub created_time: Option<String>,
}

/// 同步统计信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatistics {
    /// 总处理数量
    pub total_count: u32,
    /// 成功数量
    pub success_count: u32,
    /// 失败数量
    pub failed_count: u32,
    /// 跳过数量
    pub skipped_count: u32,
    /// 新增数量
    pub created_count: u32,
    /// 更新数量
    pub updated_count: u32,
}

/// 外部候选人数据
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalCandidate {
    /// 外部候选人ID
    pub external_id: String,
    /// 外部系统配置ID
    pub system_config_id: String,
    /// 候选人姓名
    pub name: String,
    /// 候选人邮箱
    pub email: Option<String>,
    /// 候选人电话
    pub phone: Option<String>,
    /// 简历数据
    pub resume_data: Option<serde_json::Value>,
    /// 技能标签
    pub skills: Vec<String>,
    /// 工作经验
    pub work_experience: Option<u32>,
    /// 教育背景
    pub education: Option<String>,
    /// 外部数据
    pub external_data: Option<serde_json::Value>,
    /// 同步状态
    pub sync_status: String,
    /// 关联的内部人才ID
    pub internal_talent_id: Option<String>,
    /// 最后同步时间
    pub last_sync_time: Option<String>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 外部系统配置创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalSystemConfigCreateRequest {
    /// 系统名称
    pub system_name: String,
    /// 系统类型
    pub system_type: String,
    /// 系统描述
    pub description: Option<String>,
    /// 接口地址
    pub api_endpoint: String,
    /// 认证方式
    pub auth_type: String,
    /// 认证配置
    pub auth_config: serde_json::Value,
    /// 数据映射配置
    pub data_mapping: Option<serde_json::Value>,
    /// 同步频率
    pub sync_frequency: Option<String>,
    /// 是否启用
    pub enabled: Option<bool>,
}

/// 同步任务创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SyncTaskCreateRequest {
    /// 外部系统配置ID
    pub system_config_id: String,
    /// 同步类型
    pub sync_type: String,
    /// 同步参数
    pub sync_params: Option<serde_json::Value>,
}

/// 外部候选人导入请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalCandidateImportRequest {
    /// 外部系统配置ID
    pub system_config_id: String,
    /// 外部候选人数据列表
    pub candidates: Vec<ExternalCandidateData>,
}

/// 外部候选人数据
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalCandidateData {
    /// 外部候选人ID
    pub external_id: String,
    /// 候选人姓名
    pub name: String,
    /// 候选人邮箱
    pub email: Option<String>,
    /// 候选人电话
    pub phone: Option<String>,
    /// 简历数据
    pub resume_data: Option<serde_json::Value>,
    /// 技能标签
    pub skills: Vec<String>,
    /// 工作经验
    pub work_experience: Option<u32>,
    /// 教育背景
    pub education: Option<String>,
    /// 外部数据
    pub external_data: Option<serde_json::Value>,
}

/// 外部系统配置列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemConfigListResponse {
    /// 外部系统配置列表
    #[serde(flatten)]
    pub configs: PageResponse<ExternalSystemConfig>,
}

impl ApiResponseTrait for ExternalSystemConfigListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 外部系统同步记录列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemSyncRecordListResponse {
    /// 同步记录列表
    #[serde(flatten)]
    pub records: PageResponse<ExternalSystemSyncRecord>,
}

impl ApiResponseTrait for ExternalSystemSyncRecordListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 外部候选人列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalCandidateListResponse {
    /// 外部候选人列表
    #[serde(flatten)]
    pub candidates: PageResponse<ExternalCandidate>,
}

impl ApiResponseTrait for ExternalCandidateListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 外部系统操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSystemOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 相关ID
    pub id: Option<String>,
    /// 统计信息
    pub statistics: Option<SyncStatistics>,
}

impl ApiResponseTrait for ExternalSystemOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ExternalSystemService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建外部系统配置
    ///
    /// 该接口用于创建外部系统对接配置，设置系统基本信息、
    /// 接口地址、认证方式、数据映射等配置。创建成功后
    /// 可以进行数据同步和候选人导入。
    ///
    /// # 参数
    ///
    /// - `request`: 外部系统配置创建请求参数，包括：
    ///   - `system_name`: 系统名称（必填）
    ///   - `system_type`: 系统类型（必填）
    ///   - `api_endpoint`: 接口地址（必填）
    ///   - `auth_type`: 认证方式（必填）
    ///   - `auth_config`: 认证配置（必填）
    ///   - `description`: 系统描述
    ///   - `data_mapping`: 数据映射配置
    ///   - `sync_frequency`: 同步频率
    ///   - `enabled`: 是否启用
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回外部系统配置创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `id`: 创建的配置ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::external_system::ExternalSystemConfigCreateRequest;
    /// use serde_json::json;
    ///
    /// let request = ExternalSystemConfigCreateRequest {
    ///     system_name: "智联招聘".to_string(),
    ///     system_type: "job_board".to_string(),
    ///     description: Some("智联招聘平台对接".to_string()),
    ///     api_endpoint: "https://api.zhaopin.com/v1".to_string(),
    ///     auth_type: "api_key".to_string(),
    ///     auth_config: json!({
    ///         "api_key": "your_api_key_here",
    ///         "secret": "your_secret_here"
    ///     }),
    ///     data_mapping: Some(json!({
    ///         "name_field": "candidate_name",
    ///         "email_field": "email_address",
    ///         "phone_field": "mobile_phone"
    ///     })),
    ///     sync_frequency: Some("daily".to_string()),
    ///     enabled: Some(true),
    /// };
    ///
    /// let response = client.hire.get_candidates.external_system.create_system_config(request, None).await?;
    /// ```
    pub async fn create_system_config(
        &self,
        request: ExternalSystemConfigCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExternalSystemOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_EXTERNAL_SYSTEMS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取外部系统配置列表
    ///
    /// 该接口用于获取已配置的外部系统列表，支持按系统类型、
    /// 启用状态等条件筛选。返回的列表包含系统基本信息，
    /// 可用于系统管理和状态监控。
    ///
    /// # 参数
    ///
    /// - `page_size`: 分页大小（可选），最大值100
    /// - `page_token`: 分页标记（可选）
    /// - `system_type`: 系统类型筛选（可选）
    /// - `enabled`: 启用状态筛选（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的外部系统配置列表，包括：
    /// - 系统基本信息列表
    /// - 配置和状态信息
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.hire.get_candidates.external_system.list_system_configs(
    ///     Some(20),
    ///     None,
    ///     Some("job_board".to_string()),
    ///     Some(true),
    ///     None
    /// ).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("外部系统总数: {}", data.configs.items.len());
    ///     for config in &data.configs.items {
    ///         println!("系统: {} ({})", config.system_name, if config.enabled { "启用" } else { "禁用" });
    ///     }
    /// }
    /// ```
    pub async fn list_system_configs(
        &self,
        page_size: Option<u32>,
        page_token: Option<String>,
        system_type: Option<String>,
        enabled: Option<bool>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExternalSystemConfigListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_EXTERNAL_SYSTEMS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        // 添加查询参数
        if let Some(page_size) = page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(system_type) = system_type {
            api_req.query_params.insert("system_type", system_type);
        }

        if let Some(enabled) = enabled {
            api_req.query_params.insert("enabled", enabled.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 创建同步任务
    ///
    /// 该接口用于创建外部系统数据同步任务，支持手动
    /// 触发数据同步和候选人信息拉取。
    ///
    /// # 参数
    ///
    /// - `request`: 同步任务创建请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::external_system::SyncTaskCreateRequest;
    /// use serde_json::json;
    ///
    /// let request = SyncTaskCreateRequest {
    ///     system_config_id: "sys_123456".to_string(),
    ///     sync_type: "full_sync".to_string(),
    ///     sync_params: Some(json!({
    ///         "date_range": {
    ///             "start_date": "2024-01-01",
    ///             "end_date": "2024-01-31"
    ///         },
    ///         "job_categories": ["技术", "产品"]
    ///     })),
    /// };
    ///
    /// let response = client.hire.get_candidates.external_system.create_sync_task(request, None).await?;
    /// ```
    pub async fn create_sync_task(
        &self,
        request: SyncTaskCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExternalSystemOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_EXTERNAL_SYSTEMS_SYNC_TASKS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取同步记录列表
    ///
    /// 该接口用于获取外部系统的同步记录列表，包括同步状态、
    /// 统计信息、错误日志等，用于监控同步任务的执行情况。
    ///
    /// # 参数
    ///
    /// - `system_config_id`: 外部系统配置ID（可选）
    /// - `sync_type`: 同步类型筛选（可选）
    /// - `status`: 同步状态筛选（可选）
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的同步记录列表，包括：
    /// - 同步记录基本信息列表
    /// - 同步状态和统计数据
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.hire.get_candidates.external_system.list_sync_records(
    ///     Some("sys_123456".to_string()),
    ///     Some("full_sync".to_string()),
    ///     Some("success".to_string()),
    ///     Some(20),
    ///     None,
    ///     None
    /// ).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("同步记录总数: {}", data.records.items.len());
    ///     for record in &data.records.items {
    ///         if let Some(stats) = &record.sync_statistics {
    ///             println!("同步: {} 成功: {}/{}", record.id, stats.success_count, stats.total_count);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn list_sync_records(
        &self,
        system_config_id: Option<String>,
        sync_type: Option<String>,
        status: Option<String>,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExternalSystemSyncRecordListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_EXTERNAL_SYSTEMS_SYNC_RECORDS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        // 添加查询参数
        if let Some(system_config_id) = system_config_id {
            api_req
                .query_params
                .insert("system_config_id", system_config_id);
        }

        if let Some(sync_type) = sync_type {
            api_req.query_params.insert("sync_type", sync_type);
        }

        if let Some(status) = status {
            api_req.query_params.insert("status", status);
        }

        if let Some(page_size) = page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 导入外部候选人
    ///
    /// 该接口用于批量导入外部系统的候选人数据，
    /// 支持数据映射和去重处理。
    ///
    /// # 参数
    ///
    /// - `request`: 外部候选人导入请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::get_candidates::external_system::{
    ///     ExternalCandidateImportRequest, ExternalCandidateData
    /// };
    /// use serde_json::json;
    ///
    /// let request = ExternalCandidateImportRequest {
    ///     system_config_id: "sys_123456".to_string(),
    ///     candidates: vec![
    ///         ExternalCandidateData {
    ///             external_id: "ext_001".to_string(),
    ///             name: "张三".to_string(),
    ///             email: Some("zhangsan@example.com".to_string()),
    ///             phone: Some("13800138000".to_string()),
    ///             skills: vec!["Java".to_string(), "Spring".to_string()],
    ///             work_experience: Some(5),
    ///             education: Some("本科".to_string()),
    ///             external_data: Some(json!({
    ///                 "source_platform": "zhaopin",
    ///                 "profile_url": "https://zhaopin.com/profile/001"
    ///             })),
    ///             ..Default::default()
    ///         },
    ///     ],
    /// };
    ///
    /// let response = client.hire.get_candidates.external_system.import_external_candidates(request, None).await?;
    /// ```
    pub async fn import_external_candidates(
        &self,
        request: ExternalCandidateImportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExternalSystemOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_IMPORT.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取外部候选人列表
    ///
    /// 该接口用于获取从外部系统同步的候选人列表，支持按
    /// 系统、同步状态、技能等条件筛选。返回的列表包含
    /// 候选人基本信息和同步状态。
    ///
    /// # 参数
    ///
    /// - `system_config_id`: 外部系统配置ID（可选）
    /// - `sync_status`: 同步状态筛选（可选）
    /// - `skills`: 技能标签筛选（可选）
    /// - `page_size`: 分页大小（可选）
    /// - `page_token`: 分页标记（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的外部候选人列表，包括：
    /// - 候选人基本信息列表
    /// - 同步状态和外部数据
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.hire.get_candidates.external_system.list_external_candidates(
    ///     Some("sys_123456".to_string()),
    ///     Some("synced".to_string()),
    ///     Some("Java".to_string()),
    ///     Some(50),
    ///     None,
    ///     None
    /// ).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("外部候选人总数: {}", data.candidates.items.len());
    ///     for candidate in &data.candidates.items {
    ///         println!("候选人: {} ({}年经验)", candidate.name, candidate.work_experience.unwrap_or(0));
    ///     }
    /// }
    /// ```
    pub async fn list_external_candidates(
        &self,
        system_config_id: Option<String>,
        sync_status: Option<String>,
        skills: Option<String>,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExternalCandidateListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        // 添加查询参数
        if let Some(system_config_id) = system_config_id {
            api_req
                .query_params
                .insert("system_config_id", system_config_id);
        }

        if let Some(sync_status) = sync_status {
            api_req.query_params.insert("sync_status", sync_status);
        }

        if let Some(skills) = skills {
            api_req.query_params.insert("skills", skills);
        }

        if let Some(page_size) = page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 将外部候选人转换为内部人才
    ///
    /// 该接口用于将外部系统的候选人数据转换为内部
    /// 人才档案，建立关联关系便于后续管理。
    ///
    /// # 参数
    ///
    /// - `external_candidate_id`: 外部候选人ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let external_candidate_id = "ext_cand_123456";
    /// let response = client.hire.get_candidates.external_system.convert_external_candidate(
    ///     external_candidate_id,
    ///     None
    /// ).await?;
    /// ```
    pub async fn convert_external_candidate(
        &self,
        external_candidate_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExternalSystemOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_CONVERT.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 测试外部系统连接
    ///
    /// 该接口用于测试外部系统的连接和认证配置，
    /// 验证系统配置是否正确。
    ///
    /// # 参数
    ///
    /// - `system_config_id`: 外部系统配置ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let system_config_id = "sys_123456";
    /// let response = client.hire.get_candidates.external_system.test_system_connection(
    ///     system_config_id,
    ///     None
    /// ).await?;
    /// ```
    pub async fn test_system_connection(
        &self,
        system_config_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ExternalSystemOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
                api_req.set_api_path(HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_CONVERT.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}
