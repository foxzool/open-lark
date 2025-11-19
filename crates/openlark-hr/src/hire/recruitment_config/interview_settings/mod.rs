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

use crate::hire::models::{CommonResponse, I18nText, PageResponse};

/// 面试设置服务
pub struct InterviewSettingsService {
    pub config: Config,
}

/// 面试设置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewSettings {
    /// 设置ID
    pub id: String,
    /// 设置名称
    pub name: I18nText,
    /// 设置描述
    pub description: Option<I18nText>,
    /// 面试类型
    pub interview_type: String,
    /// 面试时长配置
    pub duration_config: InterviewDurationConfig,
    /// 面试官配置
    pub interviewer_config: InterviewerConfig,
    /// 评分配置
    pub evaluation_config: Option<EvaluationConfig>,
    /// 通知配置
    pub notification_config: Option<NotificationConfig>,
    /// 是否默认设置
    pub is_default: bool,
    /// 状态
    pub status: String,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 面试时长配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterviewDurationConfig {
    // TODO: Add fields
}

/// 面试官配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterviewerConfig {
    // TODO: Add fields
}

/// 评分配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EvaluationConfig {
    // TODO: Add fields
}

/// 评分项目
#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationItem {
    /// 项目ID
    pub id: String,
    /// 项目名称
    pub name: I18nText,
    /// 项目权重
    pub weight: Option<f32>,
    /// 是否必填
    pub is_required: bool,
}

/// 评分范围
#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreRange {
    /// 最小分数
    pub min_score: f32,
    /// 最大分数
    pub max_score: f32,
    /// 分数步长
    pub step: Option<f32>,
}

/// 通知配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NotificationConfig {
    // TODO: Add fields
}

/// 面试设置创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterviewSettingsCreateRequest {
    /// 设置名称
    pub name: I18nText,
    /// 设置描述
    pub description: Option<I18nText>,
    /// 面试类型
    pub interview_type: String,
    /// 面试时长配置
    pub duration_config: InterviewDurationConfig,
    /// 面试官配置
    pub interviewer_config: InterviewerConfig,
    /// 评分配置
    pub evaluation_config: Option<EvaluationConfig>,
    /// 通知配置
    pub notification_config: Option<NotificationConfig>,
    /// 是否默认设置
    pub is_default: Option<bool>,
}

/// 面试设置列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterviewSettingsListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 面试类型
    pub interview_type: Option<String>,
    /// 状态
    pub status: Option<String>,
}

/// 面试设置列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewSettingsListResponse {
    /// 面试设置列表
    #[serde(flatten)]
    pub settings: PageResponse<InterviewSettings>,
}

impl ApiResponseTrait for InterviewSettingsListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 面试设置详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewSettingsDetailResponse {
    /// 面试设置信息
    pub settings: InterviewSettings,
}

impl ApiResponseTrait for InterviewSettingsDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 面试设置操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewSettingsOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 设置ID
    pub settings_id: Option<String>,
}

impl ApiResponseTrait for InterviewSettingsOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl InterviewSettingsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建面试设置
    ///
    /// 该接口用于创建新的面试设置模板，定义面试的
    /// 时长、面试官要求、评分标准、通知方式等配置。
    /// 创建的设置可应用于不同的面试场景。
    ///
    /// # 参数
    ///
    /// - `request`: 面试设置创建请求参数，包括：
    ///   - `name`: 设置名称（必填）
    ///   - `interview_type`: 面试类型（必填）
    ///   - `duration_config`: 时长配置（必填）
    ///   - `interviewer_config`: 面试官配置（必填）
    ///   - `description`: 设置描述
    ///   - `evaluation_config`: 评分配置
    ///   - `notification_config`: 通知配置
    ///   - `is_default`: 是否默认设置
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回面试设置创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `settings_id`: 创建的设置ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::interview_settings::{
    ///     InterviewSettingsCreateRequest, InterviewDurationConfig, InterviewerConfig
    /// };
    /// use open_lark::crate::hire::models::{I18nText, UserId};
    ///
    /// let request = InterviewSettingsCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("技术面试设置".to_string()),
    ///         en_us: Some("Technical Interview Settings".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     interview_type: "technical".to_string(),
    ///     duration_config: InterviewDurationConfig::default(),
    ///     interviewer_config: InterviewerConfig::default(),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.interview_settings.create_settings(request, None).await?;
    /// ```
    pub async fn create_settings(
        &self,
        request: InterviewSettingsCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewSettingsOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_INTERVIEW_SETTINGS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取面试设置详情
    ///
    /// 该接口用于获取指定面试设置的详细信息，包括
    /// 面试配置、评分标准、通知设置等完整数据。
    ///
    /// # 参数
    ///
    /// - `settings_id`: 面试设置ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回面试设置详细信息，包括：
    /// - 设置基本信息（名称、类型、状态等）
    /// - 面试时长配置
    /// - 面试官要求配置
    /// - 评分标准配置
    /// - 通知设置配置
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let settings_id = "settings_123456";
    /// let response = client.hire.recruitment_config.interview_settings.get_settings_detail(settings_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("设置名称: {:?}", data.settings.name.zh_cn);
    ///     println!("面试类型: {}", data.settings.interview_type);
    ///     println!("默认时长: {}分钟", data.settings.duration_config.default_duration);
    /// }
    /// ```
    pub async fn get_settings_detail(
        &self,
        settings_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewSettingsDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_INTERVIEW_SETTING_GET.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取面试设置列表
    ///
    /// 该接口用于获取企业的面试设置列表，支持按类型、
    /// 状态等条件筛选。返回的列表包含设置基本信息，
    /// 可用于设置管理和选择。
    ///
    /// # 参数
    ///
    /// - `request`: 面试设置列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `interview_type`: 面试类型筛选
    ///   - `status`: 设置状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的面试设置列表，包括：
    /// - 设置基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::interview_settings::InterviewSettingsListRequest;
    ///
    /// let request = InterviewSettingsListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     interview_type: Some("technical".to_string()),
    ///     status: Some("active".to_string()),
    /// };
    ///
    /// let response = client.hire.recruitment_config.interview_settings.list_settings(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("设置总数: {}", data.settings.items.len());
    ///     for settings in &data.settings.items {
    ///         println!("设置: {:?} (类型: {})", settings.name.zh_cn, settings.interview_type);
    ///     }
    /// }
    /// ```
    pub async fn list_settings(
        &self,
        request: InterviewSettingsListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewSettingsListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_INTERVIEW_SETTINGS.to_string());
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

        if let Some(interview_type) = request.interview_type {
            api_req
                .query_params
                .insert("interview_type", interview_type);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新面试设置
    ///
    /// 该接口用于更新现有面试设置的配置，支持修改
    /// 时长要求、面试官配置、评分标准等信息。
    ///
    /// # 参数
    ///
    /// - `settings_id`: 面试设置ID
    /// - `request`: 面试设置更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::interview_settings::{
    ///     InterviewSettingsCreateRequest, InterviewDurationConfig, InterviewerConfig
    /// };
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let settings_id = "settings_123456";
    /// let request = InterviewSettingsCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("高级技术面试设置".to_string()),
    ///         en_us: Some("Senior Technical Interview Settings".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     interview_type: "senior_technical".to_string(),
    ///     duration_config: InterviewDurationConfig::default(),
    ///     interviewer_config: InterviewerConfig::default(),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.interview_settings.update_settings(settings_id, request, None).await?;
    /// ```
    pub async fn update_settings(
        &self,
        settings_id: &str,
        request: InterviewSettingsCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewSettingsOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_INTERVIEW_SETTING_UPDATE,
            "interview_setting_id",
            settings_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 删除面试设置
    ///
    /// 该接口用于删除指定的面试设置。删除后的设置
    /// 将不再可用于面试安排。
    ///
    /// # 参数
    ///
    /// - `settings_id`: 面试设置ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let settings_id = "settings_123456";
    /// let response = client.hire.recruitment_config.interview_settings.delete_settings(settings_id, None).await?;
    /// ```
    pub async fn delete_settings(
        &self,
        settings_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<InterviewSettingsOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_INTERVIEW_SETTING_DELETE,
            "interview_setting_id",
            settings_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}
