use crate::{
    core::{
        api_resp::BaseResponse, config::Config, constants::AccessTokenType, http::Transport,
        trait_system::Service, SDKResult,
    },
    service::payroll::models::{
        PageResponse, PaymentActivity, PaymentActivityArchiveRequest, PaymentActivityListRequest,
    },
};
use open_lark_core::core::api_req::ApiRequest;
use reqwest::Method;

/// 发薪活动服务
pub struct PaymentActivityService {
    pub config: Config,
}

/// 发薪活动列表响应
#[derive(Debug, Clone)]
pub struct PaymentActivityListResponse {
    /// 发薪活动列表
    pub items: Vec<PaymentActivity>,
    /// 分页信息
    pub page: PageResponse<PaymentActivity>,
}

/// 发薪活动响应
#[derive(Debug, Clone)]
pub struct PaymentActivityResponse {
    /// 发薪活动信息
    pub data: PaymentActivity,
}

/// 封存操作响应
#[derive(Debug, Clone)]
pub struct ArchiveResponse {
    /// 操作结果
    pub success: bool,
    /// 消息
    pub message: String,
}

impl Service for PaymentActivityService {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl PaymentActivityService {
    /// 创建发薪活动服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询发薪活动列表
    ///
    /// # API文档
    ///
    /// 查询发薪活动列表，支持分页和筛选功能。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请求参数
    ///
    /// # 返回值
    ///
    /// 返回发薪活动列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::payment_activity::*;
    ///
    /// let request = PaymentActivityListRequest {
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    ///     paygroup_id: Some("pg123".to_string()),
    ///     period_start: Some("2024-01-01".to_string()),
    ///     period_end: Some("2024-01-31".to_string()),
    /// };
    ///
    /// let response = service.list_payment_activities(&request).await?;
    /// println!("找到 {} 个发薪活动", response.page.items.len());
    /// ```
    pub async fn list_payment_activities(
        &self,
        request: &PaymentActivityListRequest,
    ) -> SDKResult<BaseResponse<PaymentActivityListResponse>> {
        let mut query_params = std::collections::HashMap::new();

        if let Some(page_size) = &request.page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token".to_string(), page_token.clone());
        }
        if let Some(status) = &request.status {
            query_params.insert("status".to_string(), status.clone());
        }
        if let Some(paygroup_id) = &request.paygroup_id {
            query_params.insert("paygroup_id".to_string(), paygroup_id.clone());
        }
        if let Some(period_start) = &request.period_start {
            query_params.insert("period_start".to_string(), period_start.clone());
        }
        if let Some(period_end) = &request.period_end {
            query_params.insert("period_end".to_string(), period_end.clone());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: self
                .config
                .build_url("/open-apis/payroll/v4/payment_activities"),
            query_params,
            path_params: std::collections::HashMap::new(),
            body: None,
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 获取发薪活动详情
    ///
    /// # API文档
    ///
    /// 根据发薪活动ID获取详细信息。
    ///
    /// # 参数
    ///
    /// * `payment_activity_id` - 发薪活动ID
    ///
    /// # 返回值
    ///
    /// 返回发薪活动的详细信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::payment_activity::*;
    ///
    /// let response = service.get_payment_activity("pa_123456").await?;
    /// println!("活动名称: {:?}", response.data.activity_name.zh_cn);
    /// ```
    pub async fn get_payment_activity(
        &self,
        payment_activity_id: &str,
    ) -> SDKResult<BaseResponse<PaymentActivityResponse>> {
        let mut path_params = std::collections::HashMap::new();
        path_params.insert(
            "payment_activity_id".to_string(),
            payment_activity_id.to_string(),
        );

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: self
                .config
                .build_url("/open-apis/payroll/v4/payment_activities/{payment_activity_id}"),
            query_params: std::collections::HashMap::new(),
            path_params,
            body: None,
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 封存发薪活动
    ///
    /// # API文档
    ///
    /// 将指定的发薪活动进行封存操作，封存后的活动不能修改。
    ///
    /// # 参数
    ///
    /// * `request` - 封存请求参数
    ///
    /// # 返回值
    ///
    /// 返回封存操作的执行结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::payment_activity::*;
    ///
    /// let request = PaymentActivityArchiveRequest {
    ///     payment_activity_id: "pa_123456".to_string(),
    ///     archive_reason: Some("月度结账需要封存".to_string()),
    /// };
    ///
    /// let response = service.archive_payment_activity(&request).await?;
    /// if response.data.success {
    ///     println!("活动封存成功");
    /// }
    /// ```
    pub async fn archive_payment_activity(
        &self,
        request: &PaymentActivityArchiveRequest,
    ) -> SDKResult<BaseResponse<ArchiveResponse>> {
        let mut path_params = std::collections::HashMap::new();
        path_params.insert(
            "payment_activity_id".to_string(),
            request.payment_activity_id.clone(),
        );

        let mut body_data = serde_json::Map::new();
        if let Some(reason) = &request.archive_reason {
            body_data.insert(
                "archive_reason".to_string(),
                serde_json::Value::String(reason.clone()),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: self.config.build_url(
                "/open-apis/payroll/v4/payment_activities/{payment_activity_id}/archive",
            ),
            query_params: std::collections::HashMap::new(),
            path_params,
            body: Some(serde_json::Value::Object(body_data)),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }

    /// 取消发薪活动
    ///
    /// # API文档
    ///
    /// 取消未开始的发薪活动。
    ///
    /// # 参数
    ///
    /// * `payment_activity_id` - 发薪活动ID
    ///
    /// # 返回值
    ///
    /// 返回取消操作的执行结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::payroll::payment_activity::*;
    ///
    /// let response = service.cancel_payment_activity("pa_123456").await?;
    /// if response.data.success {
    ///     println!("活动取消成功");
    /// }
    /// ```
    pub async fn cancel_payment_activity(
        &self,
        payment_activity_id: &str,
    ) -> SDKResult<BaseResponse<ArchiveResponse>> {
        let mut path_params = std::collections::HashMap::new();
        path_params.insert(
            "payment_activity_id".to_string(),
            payment_activity_id.to_string(),
        );

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: self
                .config
                .build_url("/open-apis/payroll/v4/payment_activities/{payment_activity_id}/cancel"),
            query_params: std::collections::HashMap::new(),
            path_params,
            body: Some(serde_json::Value::Object(serde_json::Map::new())),
            supported_access_token_types: vec![AccessTokenType::Tenant],
        };

        let response = Transport::request(api_req, &self.config).await?;
        Ok(response.into())
    }
}
