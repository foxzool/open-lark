//! 更新禁用状态变更通知
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch-4

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_RESERVE_CONFIGS;

/// 更新禁用状态变更通知请求
pub struct PatchDisableInformRequest {
    config: Config,
    reserve_config_id: String,
}

impl PatchDisableInformRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_config_id: String::new(),
        }
    }

    /// 预定配置 ID（路径参数）
    pub fn reserve_config_id(mut self, reserve_config_id: impl Into<String>) -> Self {
        self.reserve_config_id = reserve_config_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch-4
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_config_id, "reserve_config_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(format!(
            "{}/{}/disable_inform",
            VC_V1_RESERVE_CONFIGS, self.reserve_config_id
        ))
        .body(serialize_params(&body, "更新禁用状态变更通知")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新禁用状态变更通知")
    }
}

