//! 批量查询日历信息
//!
//! docPath: https://open.feishu.cn/document/calendar-v4/calendar/mget-3

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::{extract_response_data, serialize_params}, endpoints::CALENDAR_V4_CALENDARS};

/// 批量查询日历信息请求
pub struct MgetCalendarRequest {
    config: Config,
}

impl MgetCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/calendar-v4/calendar/mget-3
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars/mget
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!("{}/mget", CALENDAR_V4_CALENDARS))
            .body(serialize_params(&body, "批量查询日历信息")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量查询日历信息")
    }
}
