//! 获取国家地区列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-country/region-list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::MEETING_ROOM;

/// 获取国家地区列表请求
pub struct ListCountryRequest {
    config: Config,
}

impl ListCountryRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/meeting_room/country/list
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/country/list", MEETING_ROOM));
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取国家地区列表")
    }
}

