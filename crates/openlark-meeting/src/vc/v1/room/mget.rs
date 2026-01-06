//! 批量查询会议室详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/mget

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOMS;

/// 批量查询会议室详情请求
pub struct MgetRoomRequest {
    config: Config,
}

impl MgetRoomRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/mget
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/rooms/mget
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!("{}/mget", VC_V1_ROOMS))
            .body(serialize_params(&body, "批量查询会议室详情")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量查询会议室详情")
    }
}
