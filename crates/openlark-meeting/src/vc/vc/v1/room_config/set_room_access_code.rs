//! 创建会议室部署码
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set_room_access_code

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOM_CONFIGS;

/// 创建会议室部署码请求
pub struct SetRoomAccessCodeRequest {
    config: Config,
}

impl SetRoomAccessCodeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set_room_access_code
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/room_configs/set_room_access_code
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!(
            "{}/set_room_access_code",
            VC_V1_ROOM_CONFIGS
        ))
        .body(serialize_params(&body, "创建会议室部署码")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建会议室部署码")
    }
}

