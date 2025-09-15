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
        SDKResult,
    },
    service::payroll::models::{
        PageResponse, PaymentDetail, PaymentDetailListRequest, PaymentDetailQueryRequest,
    },
};

/// 发薪明细服务
pub struct PaymentDetailService {
    pub config: Config,
}

/// 发薪明细列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetailListResponse {
    /// 发薪明细列表
    #[serde(flatten)]
    pub details: PageResponse<PaymentDetail>,
}

impl ApiResponseTrait for PaymentDetailListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发薪明细批量查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetailQueryResponse {
    /// 发薪明细列表
    pub payment_details: Vec<PaymentDetail>,
}

impl ApiResponseTrait for PaymentDetailQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PaymentDetailService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询发薪活动明细列表
    ///
    /// 该接口用于查询指定发薪活动的明细列表，支持按员工ID筛选、
    /// 分页查询等功能。可以获取员工的发薪项目详情、金额信息、
    /// 发薪状态等完整的发薪明细数据。
    ///
    /// # 参数
    ///
    /// - `request`: 发薪明细列表查询请求参数，包括：
    ///   - `payment_activity_id`: 发薪活动ID（必填）
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `employee_id`: 员工ID筛选
    ///   - `user_id_type`: 用户ID类型
    ///   - `department_id_type`: 部门ID类型
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的发薪明细列表，包括：
    /// - 员工基本信息（姓名、工号、部门、职位等）
    /// - 发薪项目详情（算薪项、金额、货币类型等）
    /// - 发薪状态和时间信息
    /// - 总发薪金额和备注信息
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::PaymentDetailListRequest;
    ///
    /// let request = PaymentDetailListRequest {
    ///     payment_activity_id: "activity_123456".to_string(),
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     employee_id: Some("emp_789".to_string()),
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("open_department_id".to_string()),
    /// };
    ///
    /// let response = client.payroll.payment_detail.list_details(request, None).await?;
    /// ```
    pub async fn list_details(
        &self,
        request: PaymentDetailListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PaymentDetailListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::PAYROLL_V1_PAYMENT_DETAILS,
                "payment_activity_id",
                &request.payment_activity_id,
            ),
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

        if let Some(employee_id) = request.employee_id {
            api_req.query_params.insert("employee_id", employee_id);
        }

        if let Some(user_id_type) = request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type);
        }

        if let Some(department_id_type) = request.department_id_type {
            api_req
                .query_params
                .insert("department_id_type", department_id_type);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量查询发薪明细
    ///
    /// 该接口用于批量查询指定员工的发薪明细信息，支持指定查询字段、
    /// 批量获取多个员工的发薪数据。适用于批量数据导出和报表生成场景。
    ///
    /// # 参数
    ///
    /// - `request`: 发薪明细批量查询请求参数，包括：
    ///   - `payment_activity_id`: 发薪活动ID（必填）
    ///   - `employee_ids`: 员工ID列表（必填）
    ///   - `user_id_type`: 用户ID类型
    ///   - `fields`: 查询的字段列表
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回批量查询的发薪明细列表，包括：
    /// - 指定员工的完整发薪明细信息
    /// - 根据fields参数返回指定字段数据
    /// - 发薪项目详情和金额统计
    /// - 发薪状态和处理时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::PaymentDetailQueryRequest;
    ///
    /// let request = PaymentDetailQueryRequest {
    ///     payment_activity_id: "activity_123456".to_string(),
    ///     employee_ids: vec![
    ///         "emp_001".to_string(),
    ///         "emp_002".to_string(),
    ///         "emp_003".to_string(),
    ///     ],
    ///     user_id_type: Some("open_id".to_string()),
    ///     fields: Some(vec![
    ///         "employee_name".to_string(),
    ///         "payment_items".to_string(),
    ///         "total_amount".to_string(),
    ///         "payment_status".to_string(),
    ///     ]),
    /// };
    ///
    /// let response = client.payroll.payment_detail.query_details(request, None).await?;
    /// ```
    pub async fn query_details(
        &self,
        request: PaymentDetailQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PaymentDetailQueryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::PAYROLL_V1_PAYMENT_DETAILS_QUERY,
                "payment_activity_id",
                &request.payment_activity_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // 添加查询参数
        if let Some(user_id_type) = request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
