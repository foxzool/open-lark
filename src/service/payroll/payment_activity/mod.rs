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
    service::payroll::models::{
        PageResponse, PaymentActivity, PaymentActivityArchiveRequest, PaymentActivityListRequest,
    },
};

/// 发薪活动服务
pub struct PaymentActivityService {
    pub config: Config,
}

/// 发薪活动列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentActivityListResponse {
    /// 发薪活动列表
    #[serde(flatten)]
    pub activities: PageResponse<PaymentActivity>,
}

impl ApiResponseTrait for PaymentActivityListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发薪活动封存响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentActivityArchiveResponse {
    /// 封存结果
    pub success: bool,
    /// 封存时间
    pub archived_time: Option<String>,
    /// 消息
    pub message: Option<String>,
}

impl ApiResponseTrait for PaymentActivityArchiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PaymentActivityService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询发薪活动列表
    ///
    /// 该接口用于查询企业的发薪活动列表，支持按状态、薪资组、
    /// 发薪周期等条件筛选。可以获取发薪活动的基本信息、
    /// 状态、关联员工数量、发薪金额等数据。
    ///
    /// # 参数
    ///
    /// - `request`: 发薪活动列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `status`: 活动状态筛选
    ///   - `paygroup_id`: 薪资组ID筛选
    ///   - `period_start`: 发薪周期开始时间
    ///   - `period_end`: 发薪周期结束时间
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的发薪活动列表，包括：
    /// - 发薪活动基本信息（活动名称、状态、薪资组等）
    /// - 发薪周期和时间信息（周期开始结束时间、计划发薪日期等）
    /// - 统计信息（员工总数、发薪总金额等）
    /// - 创建和更新时间、创建人信息
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::PaymentActivityListRequest;
    ///
    /// let request = PaymentActivityListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    ///     paygroup_id: Some("paygroup_123".to_string()),
    ///     period_start: Some("2024-01-01".to_string()),
    ///     period_end: Some("2024-12-31".to_string()),
    /// };
    ///
    /// let response = client.payroll.payment_activity.list_activities(request, None).await?;
    /// ```
    pub async fn list_activities(
        &self,
        request: PaymentActivityListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PaymentActivityListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::PAYROLL_V1_PAYMENT_ACTIVITIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

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

        if let Some(paygroup_id) = request.paygroup_id {
            api_req.query_params.insert("paygroup_id", paygroup_id);
        }

        if let Some(period_start) = request.period_start {
            api_req.query_params.insert("period_start", period_start);
        }

        if let Some(period_end) = request.period_end {
            api_req.query_params.insert("period_end", period_end);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 封存发薪活动
    ///
    /// 该接口用于封存指定的发薪活动，封存后的活动将不能再进行
    /// 修改操作。通常在发薪完成并确认数据无误后进行封存操作，
    /// 用于数据归档和合规管理。
    ///
    /// # 参数
    ///
    /// - `request`: 发薪活动封存请求参数，包括：
    ///   - `payment_activity_id`: 发薪活动ID（必填）
    ///   - `archive_reason`: 封存原因（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回封存操作结果，包括：
    /// - `success`: 封存是否成功
    /// - `archived_time`: 封存时间
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::PaymentActivityArchiveRequest;
    ///
    /// let request = PaymentActivityArchiveRequest {
    ///     payment_activity_id: "activity_123456".to_string(),
    ///     archive_reason: Some("发薪完成，数据确认无误".to_string()),
    /// };
    ///
    /// let response = client.payroll.payment_activity.archive_activity(request, None).await?;
    /// ```
    ///
    /// # 注意事项
    ///
    /// - 封存操作不可逆，请谨慎操作
    /// - 只有特定状态的发薪活动才能进行封存
    /// - 封存后的活动数据将进入只读状态
    /// - 建议在封存前做好数据备份
    pub async fn archive_activity(
        &self,
        request: PaymentActivityArchiveRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PaymentActivityArchiveResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::PAYROLL_V1_PAYMENT_ACTIVITY_ARCHIVE,
                "payment_activity_id",
                &request.payment_activity_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for PaymentActivityService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "payment_activity"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
