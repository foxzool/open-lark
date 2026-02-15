//! 更新门禁用户

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
pub struct PatchAcsUserRequest {
    config: Arc<Config>,
    user_id: String,
    body: PatchAcsUserBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchAcsUserBody {
    pub user_name: Option<String>,
    pub department: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAcsUserResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchAcsUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchAcsUserRequest {
    pub fn new(config: Arc<Config>, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
            body: PatchAcsUserBody::default(),
        }
    }

    pub fn user_name(mut self, name: impl Into<String>) -> Self {
        self.body.user_name = Some(name.into());
        self
    }

    pub fn department(mut self, dept: impl Into<String>) -> Self {
        self.body.department = Some(dept.into());
        self
    }

    pub async fn execute(self) -> SDKResult<PatchAcsUserResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchAcsUserResponse> {
        let path = format!("/open-apis/acs/v1/users/{}", self.user_id);
        let req: ApiRequest<PatchAcsUserResponse> =
            ApiRequest::patch(&path).json(&self.body).map_err(|e| {
                openlark_core::error::CoreError::Serialization(e.to_string())
            })?;

        let _resp: openlark_core::api::Response<PatchAcsUserResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(PatchAcsUserResponse { data: None })
    }
}
