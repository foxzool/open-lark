//! 取消订阅日历变更事件
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/unsubscription

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CALENDAR_V4_CALENDARS,
};

/// 取消订阅日历变更事件请求
pub struct UnsubscriptionCalendarRequest {
    config: Config,
}

impl UnsubscriptionCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/unsubscription
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars/unsubscription
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/unsubscription", CALENDAR_V4_CALENDARS))
                .body(serialize_params(&body, "取消订阅日历变更事件")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "取消订阅日历变更事件")
    }
}
