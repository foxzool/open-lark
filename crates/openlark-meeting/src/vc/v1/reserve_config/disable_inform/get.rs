//! 查询禁用状态变更通知
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get-4

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult, req_option::RequestOption};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_RESERVE_CONFIGS;

/// 查询禁用状态变更通知请求
pub struct GetDisableInformRequest {
    config: Config,
    reserve_config_id: String,
    query_params: Vec<(String, String)>,
}

impl GetDisableInformRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_config_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 预定配置 ID（路径参数）
    pub fn reserve_config_id(mut self, reserve_config_id: impl Into<String>) -> Self {
        self.reserve_config_id = reserve_config_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get-4
    pub async fn execute(self) -> SDKResult<serde_json::Value> {

        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {

        validate_required!(self.reserve_config_id, "reserve_config_id 不能为空");

        // url: GET:/open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}/disable_inform",
            VC_V1_RESERVE_CONFIGS, self.reserve_config_id
        ));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询禁用状态变更通知")

    }

}
