//! 查询会议室日程主题和会议详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::MEETING_ROOM;

/// 查询会议室日程主题和会议详情请求
pub struct BatchGetSummaryRequest {
    config: Config,
}

impl BatchGetSummaryRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/meeting_room/summary/batch_get
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/summary/batch_get", MEETING_ROOM))
                .body(serialize_params(&body, "查询会议室日程主题和会议详情")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询会议室日程主题和会议详情")
    }
}
