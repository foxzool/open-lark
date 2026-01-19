//! 回复会议室日程实例
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/reply-meeting-room-event-instance

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 回复会议室日程实例请求
pub struct ReplyInstanceRequest {
    config: Config,
}

impl ReplyInstanceRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/reply-meeting-room-event-instance
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        let api_endpoint = MeetingRoomApi::InstanceReplyOld;
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "回复会议室日程实例")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "回复会议室日程实例")
    }
}
