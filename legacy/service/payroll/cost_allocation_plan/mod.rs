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
    service::payroll::models::{CostAllocationPlan, CostAllocationPlanListRequest, PageResponse},
};

/// 成本分摊方案服务
pub struct CostAllocationPlanService {
    pub config: Config,
}

/// 成本分摊方案列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CostAllocationPlanListResponse {
    /// 成本分摊方案列表
    #[serde(flatten)]
    pub plans: PageResponse<CostAllocationPlan>,
}

impl ApiResponseTrait for CostAllocationPlanListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CostAllocationPlanService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量查询成本分摊方案
    ///
    /// 该接口用于查询企业配置的成本分摊方案列表，包括方案的
    /// 基本信息、分摊规则、生效时间等详细配置。成本分摊方案
    /// 定义了如何将人力成本分摊到不同的成本中心或部门。
    ///
    /// # 参数
    ///
    /// - `request`: 成本分摊方案查询请求参数，包括：
    ///   - `page_size`: 分页大小
    ///   - `page_token`: 分页标记
    ///   - `status`: 方案状态筛选
    ///   - `plan_type`: 方案类型筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的成本分摊方案列表，包括：
    /// - 方案基本信息（ID、名称、类型、状态等）
    /// - 分摊规则详情（规则名称、分摊维度、分摊比例等）
    /// - 目标成本中心信息
    /// - 规则条件和约束
    /// - 生效和失效时间
    /// - 创建和更新信息
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::payroll::models::CostAllocationPlanListRequest;
    ///
    /// let request = CostAllocationPlanListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    ///     plan_type: Some("department".to_string()),
    /// };
    ///
    /// let response = client.payroll.cost_allocation_plan.list_plans(request, None).await?;
    /// ```
    pub async fn list_plans(
        &self,
        request: CostAllocationPlanListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CostAllocationPlanListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: PAYROLL_V1_COST_ALLOCATION_PLANS.to_string(),
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

        if let Some(plan_type) = request.plan_type {
            api_req.query_params.insert("plan_type", plan_type);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
