//! 取消订阅日历变更事件
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/unsubscription

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 取消订阅日历变更事件请求
pub struct CalendarUnsubscriptionRequest {
    config: Config,
}

impl CalendarUnsubscriptionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/unsubscription
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars/unsubscription
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/unsubscription", CALENDAR_V4_CALENDARS));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "取消订阅日历变更事件")
    }
}
