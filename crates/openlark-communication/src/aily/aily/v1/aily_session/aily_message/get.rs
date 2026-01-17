//! 获取 Aily 消息
//!
//! docPath: https://open.feishu.cn/document/aily-v1/aily_session-aily_message/get

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_MESSAGES};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 获取 Aily 消息请求
pub struct GetMessageRequest {
    config: Config,
    aily_session_id: String,
    aily_message_id: String,
}

impl GetMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
            aily_message_id: String::new(),
        }
    }

    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    pub fn aily_message_id(mut self, aily_message_id: impl Into<String>) -> Self {
        self.aily_message_id = aily_message_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");
        validate_required!(self.aily_message_id, "aily_message_id 不能为空");

        let url = AILY_V1_MESSAGES.replace("{aily_session_id}", &self.aily_session_id)
            + "/"
            + &self.aily_message_id;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取 Aily 消息")
    }
}
