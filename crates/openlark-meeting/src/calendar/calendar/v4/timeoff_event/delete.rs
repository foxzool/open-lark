//! 删除请假日程
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/delete

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_TIMEOFF_EVENTS};

/// 删除请假日程请求
pub struct DeleteTimeoffEventRequest {
    config: Config,
    timeoff_event_id: String,
}

impl DeleteTimeoffEventRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            timeoff_event_id: String::new(),
        }
    }

    /// 请假日程 ID（路径参数）
    pub fn timeoff_event_id(mut self, timeoff_event_id: impl Into<String>) -> Self {
        self.timeoff_event_id = timeoff_event_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/delete
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.timeoff_event_id, "timeoff_event_id 不能为空");

        // url: DELETE:/open-apis/calendar/v4/timeoff_events/:timeoff_event_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(format!(
            "{}/{}",
            CALENDAR_V4_TIMEOFF_EVENTS, self.timeoff_event_id
        ));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除请假日程")
    }
}

