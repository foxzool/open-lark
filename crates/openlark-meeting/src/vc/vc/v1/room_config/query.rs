//! 查询会议室配置
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/query

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_ROOM_CONFIGS;

/// 查询会议室配置请求
pub struct QueryRoomConfigRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl QueryRoomConfigRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/query
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/room_configs/query
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/query", VC_V1_ROOM_CONFIGS));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询会议室配置")
    }
}

