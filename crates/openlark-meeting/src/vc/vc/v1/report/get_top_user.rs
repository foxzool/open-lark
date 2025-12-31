//! 获取 Top 用户列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/report/get_top_user

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_REPORTS;

/// 获取 Top 用户列表请求
pub struct GetTopUserReportRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl GetTopUserReportRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/report/get_top_user
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/reports/get_top_user
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/get_top_user", VC_V1_REPORTS));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取 Top 用户列表")
    }
}

