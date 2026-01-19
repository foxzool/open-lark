//! 批量查询主日历日程忙闲信息
//!
//! docPath: https://open.feishu.cn/document/calendar-v4/calendar/batch

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_endpoints::CalendarApiV4,
    common::api_utils::{extract_response_data, serialize_params},
};

/// 批量查询主日历日程忙闲信息请求
pub struct BatchFreebusyRequest {
    config: Config,
}

impl BatchFreebusyRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/calendar-v4/calendar/batch
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        let api_endpoint = CalendarApiV4::FreebusyBatch;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url())
                .body(serialize_params(&body, "批量查询主日历日程忙闲信息")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量查询主日历日程忙闲信息")
    }
}
