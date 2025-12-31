//! 批量获取主日历信息
//!
//! docPath: https://open.feishu.cn/document/calendar-v4/calendar/primarys

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 批量获取主日历信息请求
pub struct PrimarysCalendarRequest {
    config: Config,
}

impl PrimarysCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/calendar-v4/calendar/primarys
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars/primarys
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!("{}/primarys", CALENDAR_V4_CALENDARS));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量获取主日历信息")
    }
}
