//! 搜索会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/search

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOMS;

/// 搜索会议室请求
pub struct SearchRoomRequest {
    config: Config,
}

impl SearchRoomRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/search
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/rooms/search
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/search", VC_V1_ROOMS))
                .body(serialize_params(&body, "搜索会议室")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "搜索会议室")
    }
}

