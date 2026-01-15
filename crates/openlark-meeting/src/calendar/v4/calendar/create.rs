//! 创建共享日历
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CalendarApiV4;

/// 创建共享日历请求
pub struct CreateCalendarRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 创建共享日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCalendarResponse {
    pub calendar: CalendarData,
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

impl ApiResponseTrait for CreateCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateCalendarRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateCalendarResponse> {
        let api_endpoint = CalendarApiV4::CalendarCreate;
        let api_request: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url()).body(body);

        let response = Transport::request(api_request, &self.config, None).await?;
        let data: CreateCalendarResponse =
            serde_json::from_value(response.data.unwrap_or_default())?;
        Ok(data)
    }
}
