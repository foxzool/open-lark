use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::okr::models::{I18nText, PageResponse, Period, PeriodStatus},
};

/// OKR 周期管理服务
pub struct PeriodService {
    pub config: Config,
}

/// 周期列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodListResponse {
    /// 周期列表
    #[serde(flatten)]
    pub periods: PageResponse<Period>,
}

impl ApiResponseTrait for PeriodListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 周期创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodCreateResponse {
    /// 创建的周期信息
    pub period: Period,
}

impl ApiResponseTrait for PeriodCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 周期状态更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodStatusUpdateResponse {
    /// 更新后的周期信息
    pub period: Period,
}

impl ApiResponseTrait for PeriodStatusUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PeriodService {
    /// 创建 OKR 周期管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建 OKR 周期
    ///
    /// 创建新的 OKR 周期，设置周期的基本信息如名称、时间范围等。
    ///
    /// # Arguments
    ///
    /// * `request` - 创建周期请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回创建的周期信息
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::okr::period::*;
    /// use open_lark::service::okr::models::I18nText;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret")
    ///         .build();
    ///
    ///     let request = PeriodCreateRequest {
    ///         name: I18nText {
    ///             zh_cn: Some("2024年第一季度".to_string()),
    ///             en_us: Some("Q1 2024".to_string()),
    ///             ja_jp: None,
    ///         },
    ///         start_time: 1704067200000, // 2024-01-01
    ///         end_time: 1711900799000,   // 2024-03-31
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.okr.period.create_period(request, None).await?;
    ///     println!("创建的周期ID: {}", response.data.unwrap().period.period_id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_period(
        &self,
        request: PeriodCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PeriodCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::OKR_V1_PERIODS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改 OKR 周期状态
    ///
    /// 修改指定周期的状态，如将草稿状态的周期激活，或结束正在进行的周期。
    ///
    /// # Arguments
    ///
    /// * `period_id` - 周期ID
    /// * `request` - 状态更新请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回更新后的周期信息
    pub async fn update_period_status(
        &self,
        period_id: &str,
        request: PeriodStatusUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PeriodStatusUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                Endpoints::OKR_V1_PERIOD_GET,
                "period_id",
                period_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取 OKR 周期列表
    ///
    /// 查询系统中的 OKR 周期列表，支持按状态筛选和分页查询。
    ///
    /// # Arguments
    ///
    /// * `request` - 查询请求参数
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回周期列表
    pub async fn list_periods(
        &self,
        request: PeriodListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PeriodListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::OKR_V1_PERIODS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(status) = request.status {
            api_req.query_params.insert("status", format!("{status:?}"));
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for PeriodService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "period"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

/// 创建周期请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeriodCreateRequest {
    /// 周期名称
    pub name: I18nText,
    /// 开始时间（毫秒时间戳）
    pub start_time: i64,
    /// 结束时间（毫秒时间戳）
    pub end_time: i64,
    /// 周期描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 初始状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PeriodStatus>,
}

/// 周期状态更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodStatusUpdateRequest {
    /// 新状态
    pub status: PeriodStatus,
    /// 更新原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 周期列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeriodListRequest {
    /// 状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PeriodStatus>,
    /// 页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
