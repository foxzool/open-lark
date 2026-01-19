//! 订阅日历变更事件
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/subscription

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_endpoints::CalendarApiV4,
    common::api_utils::{extract_response_data, serialize_params},
};

/// 订阅日历变更事件请求
pub struct SubscriptionCalendarRequest {
    config: Config,
}

impl SubscriptionCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/subscription
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        let api_endpoint = CalendarApiV4::CalendarSubscription;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url())
                .body(serialize_params(&body, "订阅日历变更事件")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "订阅日历变更事件")
    }
}
