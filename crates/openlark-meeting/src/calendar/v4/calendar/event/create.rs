//! 创建日程
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建日程请求
#[derive(Debug, Clone)]
pub struct CreateCalendarEventRequest {
    config: Config,
    calendar_id: String,
}

/// 创建日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCalendarEventResponse {
    /// 日程数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for CreateCalendarEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateCalendarEventRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateCalendarEventResponse> {
        use crate::common::api_endpoints::CalendarApiV4;

        validate_required!(self.calendar_id, "calendar_id 不能为空");

        let api_endpoint = CalendarApiV4::EventCreate(self.calendar_id);
        let req: ApiRequest<CreateCalendarEventResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "创建日程")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建日程")
    }
}

/// 创建日程请求构建器
#[derive(Debug, Clone)]
pub struct CreateCalendarEventRequestBuilder {
    request: CreateCalendarEventRequest,
}

impl CreateCalendarEventRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateCalendarEventRequest::new(config),
        }
    }

    /// 设置日历ID
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.request = self.request.calendar_id(calendar_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateCalendarEventRequest {
        self.request
    }
}
