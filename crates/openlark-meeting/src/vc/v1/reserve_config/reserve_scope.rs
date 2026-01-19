//! 查询会议室预定限制
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/reserve_scope

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult,
    req_option::RequestOption};

use crate::common::api_utils::extract_response_data;

/// 查询会议室预定限制请求
pub struct GetReserveScopeRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl GetReserveScopeRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/reserve_scope
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/reserve_configs/:reserve_config_id/reserve_scope
        // 注意：这个端点需要 reserve_config_id 参数
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/vc/v1/reserve_configs/reserve_scope");
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询会议室预定限制")
    }
}
