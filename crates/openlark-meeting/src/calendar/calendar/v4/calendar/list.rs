//! 查询日历列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list-2

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 查询日历列表请求
pub struct ListCalendarRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数（用于覆盖文档中可选 query）
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list-2
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/calendar/v4/calendars
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(CALENDAR_V4_CALENDARS);
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询日历列表")
    }
}

