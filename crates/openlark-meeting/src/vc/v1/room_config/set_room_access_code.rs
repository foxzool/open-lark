//! 创建会议室部署码
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set_room_access_code

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_endpoints::VcApiV1,
    common::api_utils::{extract_response_data, serialize_params},
};

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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set_room_access_code
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        let api_endpoint = VcApiV1::RoomConfigSetRoomAccessCode;
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "创建会议室部署码")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建会议室部署码")
    }
}
