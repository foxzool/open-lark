//! 预约会议
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::VC_V1_RESERVES,
};

/// 预约会议请求
pub struct ApplyReserveRequest {
    config: Config,
}

impl ApplyReserveRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/reserves/apply
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/apply", VC_V1_RESERVES))
                .body(serialize_params(&body, "预约会议")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "预约会议")
    }
}
