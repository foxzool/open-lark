use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::hire::models::{CommonResponse, I18nText, PageResponse, UserId},
};

/// Offer设置服务
pub struct OfferSettingsService {
    pub config: Config,
}

/// Offer设置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferSettings {
    /// 设置ID
    pub id: String,
    /// 设置名称
    pub name: I18nText,
    /// 设置描述
    pub description: Option<I18nText>,
    /// 适用职位类型
    pub applicable_job_types: Vec<String>,
    /// 薪资配置
    pub salary_config: SalaryConfig,
    /// 审批配置
    pub approval_config: ApprovalConfig,
    /// 有效期配置
    pub validity_config: ValidityConfig,
    /// 签约配置
    pub contract_config: Option<ContractConfig>,
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

/// 薪资配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SalaryConfig {
    /// 支持的薪资币种
    pub supported_currencies: Vec<String>,
    /// 默认币种
    pub default_currency: String,
    /// 薪资组成项
    pub salary_components: Vec<SalaryComponent>,
    /// 是否允许自定义薪资组成
    pub allow_custom_components: bool,
}

/// 薪资组成项
#[derive(Debug, Serialize, Deserialize)]
pub struct SalaryComponent {
    /// 组成项ID
    pub id: String,
    /// 组成项名称
    pub name: I18nText,
    /// 组成项类型
    pub component_type: String,
    /// 是否必填
    pub is_required: bool,
    /// 最小值
    pub min_value: Option<f64>,
    /// 最大值
    pub max_value: Option<f64>,
}

/// 审批配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApprovalConfig {
    /// 是否需要审批
    pub require_approval: bool,
    /// 审批流程ID
    pub approval_process_id: Option<String>,
    /// 审批人列表
    pub approvers: Vec<UserId>,
    /// 审批规则
    pub approval_rules: Vec<ApprovalRule>,
}

/// 审批规则
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalRule {
    /// 规则ID
    pub id: String,
    /// 规则名称
    pub name: String,
    /// 条件表达式
    pub condition: String,
    /// 审批人
    pub approvers: Vec<UserId>,
}

/// 有效期配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidityConfig {
    /// 默认有效期（天）
    pub default_validity_days: u32,
    /// 最小有效期（天）
    pub min_validity_days: Option<u32>,
    /// 最大有效期（天）
    pub max_validity_days: Option<u32>,
    /// 是否允许自定义有效期
    pub allow_custom_validity: bool,
}

/// 签约配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    /// 是否支持电子签约
    pub support_electronic_contract: bool,
    /// 合同模板ID
    pub contract_template_id: Option<String>,
    /// 签约平台配置
    pub signing_platform_config: Option<serde_json::Value>,
}

/// 通知配置
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// 是否发送Offer通知
    pub send_offer_notification: bool,
    /// 是否发送接受确认
    pub send_acceptance_confirmation: bool,
    /// 是否发送到期提醒
    pub send_expiry_reminder: bool,
    /// 提醒时间（到期前多少天）
    pub reminder_days: Option<u32>,
    /// 通知模板ID
    pub template_id: Option<String>,
}

/// Offer设置创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OfferSettingsCreateRequest {
    /// 设置名称
    pub name: I18nText,
    /// 设置描述
    pub description: Option<I18nText>,
    /// 适用职位类型
    pub applicable_job_types: Vec<String>,
    /// 薪资配置
    pub salary_config: SalaryConfig,
    /// 审批配置
    pub approval_config: ApprovalConfig,
    /// 有效期配置
    pub validity_config: ValidityConfig,
    /// 签约配置
    pub contract_config: Option<ContractConfig>,
    /// 通知配置
    pub notification_config: Option<NotificationConfig>,
    /// 是否默认设置
    pub is_default: Option<bool>,
}

/// Offer设置列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OfferSettingsListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 适用职位类型
    pub job_type: Option<String>,
    /// 状态
    pub status: Option<String>,
}

/// Offer设置列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferSettingsListResponse {
    /// Offer设置列表
    #[serde(flatten)]
    pub settings: PageResponse<OfferSettings>,
}

impl ApiResponseTrait for OfferSettingsListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Offer设置详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferSettingsDetailResponse {
    /// Offer设置信息
    pub settings: OfferSettings,
}

impl ApiResponseTrait for OfferSettingsDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Offer设置操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferSettingsOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 设置ID
    pub settings_id: Option<String>,
}

impl ApiResponseTrait for OfferSettingsOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl OfferSettingsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建Offer设置
    ///
    /// 该接口用于创建新的Offer设置模板，定义Offer的
    /// 薪资结构、审批流程、有效期、签约方式等配置。
    /// 创建的设置可应用于不同类型的职位Offer。
    ///
    /// # 参数
    ///
    /// - `request`: Offer设置创建请求参数，包括：
    ///   - `name`: 设置名称（必填）
    ///   - `applicable_job_types`: 适用职位类型（必填）
    ///   - `salary_config`: 薪资配置（必填）
    ///   - `approval_config`: 审批配置（必填）
    ///   - `validity_config`: 有效期配置（必填）
    ///   - `description`: 设置描述
    ///   - `contract_config`: 签约配置
    ///   - `notification_config`: 通知配置
    ///   - `is_default`: 是否默认设置
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回Offer设置创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `settings_id`: 创建的设置ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::offer_settings::{
    ///     OfferSettingsCreateRequest, SalaryConfig, ApprovalConfig, ValidityConfig
    /// };
    /// use open_lark::service::hire::models::I18nText;
    ///
    /// let request = OfferSettingsCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("技术岗位Offer设置".to_string()),
    ///         en_us: Some("Technical Position Offer Settings".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     applicable_job_types: vec!["software_engineer".to_string()],
    ///     salary_config: SalaryConfig {
    ///         supported_currencies: vec!["CNY".to_string(), "USD".to_string()],
    ///         default_currency: "CNY".to_string(),
    ///         salary_components: vec![],
    ///         allow_custom_components: true,
    ///     },
    ///     approval_config: ApprovalConfig {
    ///         require_approval: true,
    ///         approval_process_id: None,
    ///         approvers: vec![],
    ///         approval_rules: vec![],
    ///     },
    ///     validity_config: ValidityConfig {
    ///         default_validity_days: 7,
    ///         min_validity_days: Some(3),
    ///         max_validity_days: Some(30),
    ///         allow_custom_validity: true,
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.offer_settings.create_settings(request, None).await?;
    /// ```
    pub async fn create_settings(
        &self,
        request: OfferSettingsCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OfferSettingsOperationResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/hire/v1/offer_settings".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取Offer设置详情
    ///
    /// 该接口用于获取指定Offer设置的详细信息，包括
    /// 薪资配置、审批流程、有效期设置等完整数据。
    ///
    /// # 参数
    ///
    /// - `settings_id`: Offer设置ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回Offer设置详细信息，包括：
    /// - 设置基本信息（名称、适用类型、状态等）
    /// - 薪资结构配置
    /// - 审批流程配置
    /// - 有效期设置
    /// - 签约配置
    /// - 通知设置
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let settings_id = "settings_123456";
    /// let response = client.hire.recruitment_config.offer_settings.get_settings_detail(settings_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("设置名称: {:?}", data.settings.name.zh_cn);
    ///     println!("默认币种: {}", data.settings.salary_config.default_currency);
    ///     println!("默认有效期: {}天", data.settings.validity_config.default_validity_days);
    /// }
    /// ```
    pub async fn get_settings_detail(
        &self,
        settings_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OfferSettingsDetailResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/hire/v1/offer_settings/{settings_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取Offer设置列表
    ///
    /// 该接口用于获取企业的Offer设置列表，支持按职位类型、
    /// 状态等条件筛选。返回的列表包含设置基本信息，
    /// 可用于设置管理和选择。
    ///
    /// # 参数
    ///
    /// - `request`: Offer设置列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `job_type`: 适用职位类型筛选
    ///   - `status`: 设置状态筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的Offer设置列表，包括：
    /// - 设置基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::offer_settings::OfferSettingsListRequest;
    ///
    /// let request = OfferSettingsListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     job_type: Some("software_engineer".to_string()),
    ///     status: Some("active".to_string()),
    /// };
    ///
    /// let response = client.hire.recruitment_config.offer_settings.list_settings(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("设置总数: {}", data.settings.items.len());
    ///     for settings in &data.settings.items {
    ///         println!("设置: {:?} (币种: {})", settings.name.zh_cn, settings.salary_config.default_currency);
    ///     }
    /// }
    /// ```
    pub async fn list_settings(
        &self,
        request: OfferSettingsListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OfferSettingsListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/hire/v1/offer_settings".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        if let Some(job_type) = request.job_type {
            api_req
                .query_params
                .insert("job_type".to_string(), job_type);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status".to_string(), status);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新Offer设置
    ///
    /// 该接口用于更新现有Offer设置的配置，支持修改
    /// 薪资结构、审批流程、有效期等信息。
    ///
    /// # 参数
    ///
    /// - `settings_id`: Offer设置ID
    /// - `request`: Offer设置更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::offer_settings::{
    ///     OfferSettingsCreateRequest, SalaryConfig, ApprovalConfig, ValidityConfig
    /// };
    /// use open_lark::service::hire::models::I18nText;
    ///
    /// let settings_id = "settings_123456";
    /// let request = OfferSettingsCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("高级技术岗位Offer设置".to_string()),
    ///         en_us: Some("Senior Technical Position Offer Settings".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     applicable_job_types: vec!["senior_engineer".to_string()],
    ///     validity_config: ValidityConfig {
    ///         default_validity_days: 14,
    ///         min_validity_days: Some(7),
    ///         max_validity_days: Some(60),
    ///         allow_custom_validity: true,
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.offer_settings.update_settings(settings_id, request, None).await?;
    /// ```
    pub async fn update_settings(
        &self,
        settings_id: &str,
        request: OfferSettingsCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OfferSettingsOperationResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("/open-apis/hire/v1/offer_settings/{settings_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除Offer设置
    ///
    /// 该接口用于删除指定的Offer设置。删除后的设置
    /// 将不再可用于Offer发放。
    ///
    /// # 参数
    ///
    /// - `settings_id`: Offer设置ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let settings_id = "settings_123456";
    /// let response = client.hire.recruitment_config.offer_settings.delete_settings(settings_id, None).await?;
    /// ```
    pub async fn delete_settings(
        &self,
        settings_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OfferSettingsOperationResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/hire/v1/offer_settings/{settings_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
