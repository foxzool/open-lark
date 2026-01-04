//! 创建会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-meeting-room

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::{extract_response_data, serialize_params}, endpoints::MEETING_ROOM};

/// 创建会议室请求
pub struct CreateRoomRequest {
    config: Config,
}

impl CreateRoomRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-meeting-room
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/meeting_room/room/create
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/room/create", MEETING_ROOM))
                .body(serialize_params(&body, "创建会议室")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建会议室")
    }
}
