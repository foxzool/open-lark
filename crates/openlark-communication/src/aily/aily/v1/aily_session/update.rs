//! 更新会话
//!
//! docPath: https://open.feishu.cn/document/aily-v1/aily_session/update

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SESSION};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 更新会话请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateSessionBody {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// 更新会话请求
pub struct UpdateSessionRequest {
    config: Config,
    aily_session_id: String,
}

impl UpdateSessionRequest {
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

    pub async fn execute(self, body: UpdateSessionBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateSessionBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");

        let url = AILY_V1_SESSION.replace("{aily_session_id}", &self.aily_session_id);
        let req: ApiRequest<UpdateSessionBody> = ApiRequest::put(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新会话")
    }
}
