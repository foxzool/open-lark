//! 搜索会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/search

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

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
        let api_endpoint = VcApiV1::RoomSearch;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url())
                .body(serialize_params(&body, "搜索会议室")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "搜索会议室")
    }
}
