//! 获取城市列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-city-list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::MEETING_ROOM;

/// 获取城市列表请求
pub struct ListDistrictRequest {
    config: Config,
}

impl ListDistrictRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/meeting_room/district/list
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/district/list", MEETING_ROOM));
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取城市列表")
    }
}

