//! 更新日历信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/patch

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CalendarApiV4;

/// 更新日历信息请求
pub struct PatchCalendarRequest {
    config: Config,
    calendar_id: String,
}

/// 更新日历信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchCalendarResponse {
    pub calendar: CalendarData,
}

impl ApiResponseTrait for PatchCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 日历数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarData {
    /// 日历 ID
    pub calendar_id: String,
    /// 日历摘要
    pub summary: String,
    /// 日历描述
    pub description: Option<String>,
    /// 日历颜色
    pub color: Option<String>,
    /// 权限
    pub permissions: Option<CalendarPermissions>,
    /// 是否为主日历
    pub primary: Option<bool>,
    /// 日历类型
    pub calendar_type: Option<String>,
}

/// 日历权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarPermissions {
    /// 是否可读
    pub is_readable: Option<bool>,
    /// 是否可写
    pub is_writable: Option<bool>,
}

impl PatchCalendarRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/patch
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<PatchCalendarResponse> {
        if self.calendar_id.trim().is_empty() {
            return Err(validation_error("calendar_id", "日历 ID 不能为空"));
        }

        let api_endpoint = CalendarApiV4::CalendarPatch(self.calendar_id.clone());
        let api_request: ApiRequest<PatchCalendarResponse> =
            ApiRequest::patch(api_endpoint.to_url()).body(body);

        let response = Transport::request(api_request, &self.config, None).await?;
        let data: PatchCalendarResponse =
            serde_json::from_value(response.data.unwrap_or_default())?;
        Ok(data)
    }
}
