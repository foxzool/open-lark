//! 查询 Exchange 账户的绑定状态
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_EXCHANGE_BINDINGS};

/// 查询 Exchange 账户的绑定状态请求
pub struct GetExchangeBindingRequest {
    config: Config,
    exchange_binding_id: String,
}

impl GetExchangeBindingRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            exchange_binding_id: String::new(),
        }
    }

    /// Exchange 绑定 ID（路径参数）
    pub fn exchange_binding_id(mut self, exchange_binding_id: impl Into<String>) -> Self {
        self.exchange_binding_id = exchange_binding_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/get
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.exchange_binding_id, "exchange_binding_id 不能为空");

        // url: GET:/open-apis/calendar/v4/exchange_bindings/:exchange_binding_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}",
            CALENDAR_V4_EXCHANGE_BINDINGS, self.exchange_binding_id
        ));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询 Exchange 账户的绑定状态")
    }
}
