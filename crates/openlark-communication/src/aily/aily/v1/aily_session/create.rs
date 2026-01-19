//! 创建会话
//!
//! docPath: https://open.feishu.cn/document/aily-v1/aily_session/create

use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SESSIONS};

/// 创建会话请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateSessionBody {
    pub name: String,
    pub description: Option<String>,
}

/// 创建会话请求
pub struct CreateSessionRequest {
    config: Config,
}

impl CreateSessionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self, body: CreateSessionBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateSessionBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(body.name, "name 不能为空");

        let req: ApiRequest<CreateSessionBody> =
            ApiRequest::post(AILY_V1_SESSIONS).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建会话")
}
}
