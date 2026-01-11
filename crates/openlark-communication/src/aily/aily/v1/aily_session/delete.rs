//! 删除会话
//!
//! docPath: https://open.feishu.cn/document/aily-v1/aily_session/delete

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, SDKResult,
};
use openlark_core::validate_required;
use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SESSION};

/// 删除会话请求
pub struct DeleteSessionRequest {
    config: Config,
    aily_session_id: String,
}

impl DeleteSessionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
        }
    }

    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");

        let url = AILY_V1_SESSION.replace("{aily_session_id}", &self.aily_session_id);
        let req: ApiRequest<()> = ApiRequest::delete(&url);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除会话")
    }
}
