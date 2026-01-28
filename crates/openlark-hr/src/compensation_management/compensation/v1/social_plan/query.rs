//! 根据方案ID和生效日期批量查询参保方案
//!
//! docPath: https://open.feishu.cn/document/server-docs/compensation-v1/social_plan/query

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 根据方案ID和生效日期批量查询参保方案请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryRequest {
    /// 方案 ID 列表（必填）
    plan_ids: Vec<String>,
    /// 生效日期（必填）
    effective_date: i64,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, plan_ids: Vec<String>, effective_date: i64) -> Self {
        Self {
            plan_ids,
            effective_date,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        // 1. 构建端点
        let api_endpoint = CompensationApiV1::SocialPlanQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            plan_ids: self.plan_ids,
            effective_date: self.effective_date,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询参保方案响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 方案 ID 列表
    pub plan_ids: Vec<String>,
    /// 生效日期
    pub effective_date: i64,
}

/// 根据方案ID和生效日期批量查询参保方案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 参保方案列表
    pub items: Vec<SocialPlan>,
}

/// 参保方案
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPlan {
    /// 方案 ID
    pub id: String,
    /// 方案名称
    pub name: String,
    /// 生效日期
    pub effective_date: i64,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
