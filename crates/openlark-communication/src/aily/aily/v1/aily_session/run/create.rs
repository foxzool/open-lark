//! 创建运行
//!
//! docPath: https://open.feishu.cn/document/aily-v1/aily_session-run/create

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_RUNS};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 创建运行请求
pub struct CreateRunRequest {
    config: Config,
    aily_session_id: String,
}

impl CreateRunRequest {
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

    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");

        let url = AILY_V1_RUNS.replace("{aily_session_id}", &self.aily_session_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建运行")
}
}
