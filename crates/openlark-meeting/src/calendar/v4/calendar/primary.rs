//! 查询主日历信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/primary

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 查询主日历信息请求
pub struct PrimaryCalendarRequest {
    config: Config,
}

impl PrimaryCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/primary
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars/primary
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/primary", CALENDAR_V4_CALENDARS));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询主日历信息")
    }
}
