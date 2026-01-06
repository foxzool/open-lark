//! 删除会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/delete

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_ROOMS;

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

    /// 会议室 ID（路径参数）
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.room_id = room_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/delete
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.room_id, "room_id 不能为空");

        // url: DELETE:/open-apis/vc/v1/rooms/:room_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete(format!("{}/{}", VC_V1_ROOMS, self.room_id));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除会议室")
    }
}
