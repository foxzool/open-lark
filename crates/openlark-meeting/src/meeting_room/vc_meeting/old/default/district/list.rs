//! 获取城市列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-city-list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::MEETING_ROOM;

/// 获取城市列表请求
pub struct ListDistrictRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListDistrictRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数（例如：country_id）
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/meeting_room/district/list
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/district/list", MEETING_ROOM));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取城市列表")
    }
}
