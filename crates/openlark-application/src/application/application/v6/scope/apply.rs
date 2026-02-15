//! 向管理员申请授权

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ApplyScopeRequest {
    config: Arc<Config>,
    body: ApplyScopeBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplyScopeBody {
    pub scope_type: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyScopeResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ApplyScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApplyScopeRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ApplyScopeBody::default(),
        }
    }

    pub async fn execute(self) -> SDKResult<ApplyScopeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ApplyScopeResponse> {
        let path = "/open-apis/application/v6/scopes/apply";
        let req: ApiRequest<ApplyScopeResponse> =
            ApiRequest::post(path).json(&self.body).map_err(|e| {
                openlark_core::error::CoreError::Serialization(e.to_string())
            })?;

        let _resp: openlark_core::api::Response<ApplyScopeResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(ApplyScopeResponse { data: None })
    }
}
