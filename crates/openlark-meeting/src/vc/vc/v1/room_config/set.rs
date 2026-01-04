//! 设置会议室配置
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::{extract_response_data, serialize_params}, endpoints::VC_V1_ROOM_CONFIGS};

/// 设置会议室配置请求
pub struct SetRoomConfigRequest {
    config: Config,
}

impl SetRoomConfigRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/room_configs/set
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/set", VC_V1_ROOM_CONFIGS))
                .body(serialize_params(&body, "设置会议室配置")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "设置会议室配置")
    }
}
