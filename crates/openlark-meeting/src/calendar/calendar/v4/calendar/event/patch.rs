//! 更新日程
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/patch

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::{extract_response_data, serialize_params}, endpoints::CALENDAR_V4_CALENDARS};

/// 更新日程请求
pub struct PatchCalendarEventRequest {
    config: Config,
    calendar_id: String,
    event_id: String,
}

impl PatchCalendarEventRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            event_id: String::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// 日程 ID（路径参数）
    pub fn event_id(mut self, event_id: impl Into<String>) -> Self {
        self.event_id = event_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/patch
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.event_id, "event_id 不能为空");

        // url: PATCH:/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(format!(
            "{}/{}/events/{}",
            CALENDAR_V4_CALENDARS, self.calendar_id, self.event_id
        ))
        .body(serialize_params(&body, "更新日程")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新日程")
    }
}

