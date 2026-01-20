//! 删除会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-meeting-room

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::common::api_utils::extract_response_data;

/// 删除会议室请求
pub struct DeleteRoomRequest {
    config: Config,
    room_id: String,
}

impl DeleteRoomRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_id: String::new(),
        }
    }

    /// 会议室 ID
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.room_id = room_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-meeting-room
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.room_id, "room_id 不能为空");

        // url: DELETE:/open-apis/meeting_room/rooms/:room_id
        let api_endpoint = MeetingRoomApi::RoomDelete(self.room_id.clone());
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete(api_endpoint.to_url()).body(serde_json::json!({
                "room_id": self.room_id
            }));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除会议室")
    }
}
