//! 列出 Aily 消息
//!
//! docPath: https://open.feishu.cn/document/aily-v1/aily_session-aily_message/list

use std::collections::HashMap;

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_MESSAGES};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 列出 Aily 消息请求
pub struct ListAilyMessagesRequest {
    config: Config,
    aily_session_id: String,
    query: HashMap<String, String>,
}

impl ListAilyMessagesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
            query: HashMap::new(),
        }
    }

    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");

        let url = AILY_V1_MESSAGES.replace("{aily_session_id}", &self.aily_session_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        for (k, v) in self.query {
            req = req.query(&k, &v);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "列出 Aily 消息")
}
}
