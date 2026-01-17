//! 获取会议报告
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/report/get_daily

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::extract_response_data;

/// 获取会议报告请求
#[derive(Debug, Clone)]
pub struct GetDailyReportRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 获取会议报告响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDailyReportResponse {
    /// 报告数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for GetDailyReportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetDailyReportRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/report/get_daily
    pub async fn execute(self) -> SDKResult<GetDailyReportResponse> {
        use crate::common::api_endpoints::VcApiV1;

        let mut req: ApiRequest<GetDailyReportResponse> =
            ApiRequest::get(VcApiV1::ReportDailyGet.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取会议报告")
    }
}

/// 获取会议报告请求构建器
#[derive(Debug, Clone)]
pub struct GetDailyReportRequestBuilder {
    request: GetDailyReportRequest,
}

impl GetDailyReportRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetDailyReportRequest::new(config),
        }
    }

    /// 添加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.query_param(key, value);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetDailyReportRequest {
        self.request
    }
}
