//! 设置会议室配置
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_SCOPE_CONFIG;

/// 设置会议室配置请求
pub struct CreateScopeConfigRequest {
    config: Config,
}

impl CreateScopeConfigRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/scope_config
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(VC_V1_SCOPE_CONFIG).body(serialize_params(&body, "设置会议室配置")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "设置会议室配置")
    }
}
