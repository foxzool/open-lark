//! 查询主日历信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/primary

use openlark_core::{api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult};

use crate::common::{api_endpoints::CalendarApiV4, api_utils::extract_response_data};

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
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars/primary
        let api_endpoint = CalendarApiV4::CalendarPrimaryGet;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(api_endpoint.to_url());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询主日历信息")
    }
}
