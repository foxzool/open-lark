//! 查询会议室配置
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/query

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::{extract_response_data, serialize_params}, endpoints::VC_V1_ROOM_CONFIGS};

/// 查询会议室配置请求
pub struct QueryRoomConfigRequest {
    config: Config,
}

impl QueryRoomConfigRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/query
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/room_configs/query
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/query", VC_V1_ROOM_CONFIGS))
                .body(serialize_params(&body, "查询会议室配置")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询会议室配置")
    }
}
