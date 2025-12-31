//! 更新会议室预定限制
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_RESERVE_CONFIGS;

/// 更新会议室预定限制请求
pub struct PatchReserveConfigRequest {
    config: Config,
    reserve_config_id: String,
}

impl PatchReserveConfigRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/scope_config/patch
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_config_id, "reserve_config_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/reserve_configs/:reserve_config_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(format!(
            "{}/{}",
            VC_V1_RESERVE_CONFIGS, self.reserve_config_id
        ))
        .body(serialize_params(&body, "更新会议室预定限制")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新会议室预定限制")
    }
}

