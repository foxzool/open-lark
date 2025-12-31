//! 创建会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-meeting-room

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::MEETING_ROOM;

/// 创建会议室请求
pub struct CreateMeetingRoomRequest {
    config: Config,
}

impl CreateMeetingRoomRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/meeting_room/room/create
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/room/create", MEETING_ROOM))
                .body(serialize_params(&body, "创建会议室")?);
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建会议室")
    }
}

