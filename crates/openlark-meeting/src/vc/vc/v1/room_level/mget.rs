//! 批量查询会议室层级详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room_level/mget

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 批量查询会议室层级详情请求
pub struct MgetRoomLevelRequest {
    config: Config,
}

impl MgetRoomLevelRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room_level/mget
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/room_levels/mget
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/mget", VC_V1_ROOM_LEVELS))
                .body(serialize_params(&body, "批量查询会议室层级详情")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量查询会议室层级详情")
    }
}

