//! 删除共享日历
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CalendarApiV4;

/// 删除共享日历请求
pub struct DeleteCalendarRequest {
    config: Config,
    calendar_id: String,
}

/// 删除共享日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteCalendarResponse {}

impl ApiResponseTrait for DeleteCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/delete
    pub async fn execute(self) -> SDKResult<DeleteCalendarResponse> {
        if self.calendar_id.trim().is_empty() {
            return Err(validation_error("calendar_id", "日历 ID 不能为空"));
        }

        let api_endpoint = CalendarApiV4::CalendarDelete(self.calendar_id.clone());
        let api_request: ApiRequest<DeleteCalendarResponse> =
            ApiRequest::delete(api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}
