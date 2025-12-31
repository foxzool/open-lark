//! 创建会议群
//!
//! docPath: https://open.feishu.cn/document/calendar-v4/calendar-event-meeting_chat/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 创建会议群请求
pub struct CreateMeetingChatRequest {
    config: Config,
    calendar_id: String,
    event_id: String,
}

impl CreateMeetingChatRequest {
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
    /// docPath: https://open.feishu.cn/document/calendar-v4/calendar-event-meeting_chat/create
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.event_id, "event_id 不能为空");

        // url: POST:/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!(
            "{}/{}/events/{}/meeting_chat",
            CALENDAR_V4_CALENDARS, self.calendar_id, self.event_id
        ));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建会议群")
    }
}
