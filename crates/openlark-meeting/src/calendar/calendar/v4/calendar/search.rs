//! 搜索日历
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/search

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};

/// 搜索日历请求
pub struct SearchCalendarRequest {
    config: Config,
}

impl SearchCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/search
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars/search
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/calendar/v4/calendars/search")
                .body(serialize_params(&body, "搜索日历")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "搜索日历")
    }
}

