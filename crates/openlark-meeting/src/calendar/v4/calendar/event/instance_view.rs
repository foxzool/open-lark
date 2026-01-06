//! 查询日程视图
//!
//! docPath: https://open.feishu.cn/document/calendar-v4/calendar-event/instance_view

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 查询日程视图请求
pub struct InstanceViewCalendarEventRequest {
    config: Config,
    calendar_id: String,
    query_params: Vec<(String, String)>,
}

impl InstanceViewCalendarEventRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/calendar-v4/calendar-event/instance_view
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");

        // url: GET:/open-apis/calendar/v4/calendars/:calendar_id/events/instance_view
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}/events/instance_view",
            CALENDAR_V4_CALENDARS, self.calendar_id
        ));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询日程视图")
    }
}
