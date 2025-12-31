//! 删除会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-meeting-room

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::MEETING_ROOM;

/// 删除会议室请求
pub struct DeleteMeetingRoomRequest {
    config: Config,
}

impl DeleteMeetingRoomRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/meeting_room/room/delete
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/room/delete", MEETING_ROOM))
                .body(serialize_params(&body, "删除会议室")?);
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除会议室")
    }
}

