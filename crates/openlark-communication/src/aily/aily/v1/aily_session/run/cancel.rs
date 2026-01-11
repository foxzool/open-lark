//! 取消运行
//!
//! docPath: https://open.feishu.cn/document/aily-v1/aily_session-run/cancel

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, SDKResult,
};
use openlark_core::validate_required;
use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_RUN_CANCEL};

/// 取消运行请求
pub struct CancelRunRequest {
    config: Config,
    aily_session_id: String,
    run_id: String,
}

impl CancelRunRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
            run_id: String::new(),
        }
    }

    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    pub fn run_id(mut self, run_id: impl Into<String>) -> Self {
        self.run_id = run_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");
        validate_required!(self.run_id, "run_id 不能为空");

        let url = AILY_V1_RUN_CANCEL
            .replace("{aily_session_id}", &self.aily_session_id)
            .replace("{run_id}", &self.run_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(&url);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "取消运行")
    }
}
