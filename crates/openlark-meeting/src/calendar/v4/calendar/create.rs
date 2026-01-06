//! 创建共享日历
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CALENDAR_V4_CALENDARS,
};

/// 创建共享日历请求
pub struct CreateCalendarRequest {
    config: Config,
}

impl CreateCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(CALENDAR_V4_CALENDARS).body(serialize_params(&body, "创建共享日历")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建共享日历")
    }
}
