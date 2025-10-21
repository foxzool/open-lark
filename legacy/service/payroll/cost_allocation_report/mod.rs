use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::payroll::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::payroll::models::{
        CostAllocationReport, CostAllocationReportListRequest, PageResponse,
    },
};

/// 成本分摊报表服务
pub struct CostAllocationReportService {
    pub config: Config,
}

/// 成本分摊报表列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CostAllocationReportListResponse {
    /// 成本分摊报表列表
    #[serde(flatten)]
    pub reports: PageResponse<CostAllocationReport>,
}

impl ApiResponseTrait for CostAllocationReportListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CostAllocationReportService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询成本分摊报表汇总数据
    ///
    /// 该接口用于查询指定时间范围内的成本分摊报表汇总数据，
    /// 支持按成本中心、部门等维度筛选。成本分摊报表用于
    /// 财务管理和成本控制，提供精确的人力成本分摊信息。
    ///
    /// # 参数
    ///
    /// - `request`: 成本分摊报表查询请求参数，包括：
    ///   - `start_date`: 开始日期（必填）
    ///   - `end_date`: 结束日期（必填）
    ///   - `cost_center_id`: 成本中心ID筛选
    ///   - `department_id`: 部门ID筛选
    ///   - `page_size`: 分页大小
    ///   - `page_token`: 分页标记
    ///   - `report_type`: 报表类型
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的成本分摊报表列表，包括：
    /// - 成本中心和部门信息
    /// - 员工数量和总成本金额
    /// - 分摊明细（算薪项、分摊金额、分摊比例等）
    /// - 统计周期和生成时间
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::CostAllocationReportListRequest;
    ///
    /// let request = CostAllocationReportListRequest {
    ///     start_date: "2024-01-01".to_string(),
    ///     end_date: "2024-12-31".to_string(),
    ///     cost_center_id: Some("cc_123".to_string()),
    ///     department_id: Some("dept_456".to_string()),
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     report_type: Some("monthly".to_string()),
    /// };
    ///
    /// let response = client.payroll.cost_allocation_report.list_reports(request, None).await?;
    /// ```
    pub async fn list_reports(
        &self,
        request: CostAllocationReportListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CostAllocationReportListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: PAYROLL_V1_COST_ALLOCATION_REPORTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req
            .query_params
            .insert("start_date", request.start_date);

        api_req.query_params.insert("end_date", request.end_date);

        if let Some(cost_center_id) = request.cost_center_id {
            api_req
                .query_params
                .insert("cost_center_id", cost_center_id);
        }

        if let Some(department_id) = request.department_id {
            api_req.query_params.insert("department_id", department_id);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(report_type) = request.report_type {
            api_req.query_params.insert("report_type", report_type);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
