//! 创建请假日程
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CALENDAR_V4_TIMEOFF_EVENTS,
};

/// 创建请假日程请求
pub struct CreateTimeoffEventRequest {
    config: Config,
}

impl CreateTimeoffEventRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/timeoff_events
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(CALENDAR_V4_TIMEOFF_EVENTS)
            .body(serialize_params(&body, "创建请假日程")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建请假日程")
    }
}
