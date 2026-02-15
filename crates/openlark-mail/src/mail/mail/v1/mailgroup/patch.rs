//! 修改邮件组部分信息

use crate::common::api_utils::serialize_params;
use openlark_core::{
    api::Response,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PatchMailGroupRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: PatchMailGroupBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchMailGroupBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMailGroupResponse {
    pub data: Option<serde_json::Value>,
}

impl PatchMailGroupRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: PatchMailGroupBody::default(),
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.body.owner = Some(owner.into());
        self
    }

    pub async fn execute(self) -> SDKResult<PatchMailGroupResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchMailGroupResponse> {
        let path = format!("/open-apis/mail/v1/mailgroups/{}", self.mailgroup_id);
        let req: ApiRequest<PatchMailGroupResponse> =
            ApiRequest::patch(&path).body(serialize_params(&self.body, "请求")?);

        let _resp: Response<PatchMailGroupResponse> = Transport::request(req, &self.config, Some(option)).await?;
        Ok(PatchMailGroupResponse { data: None })
    }
}

impl openlark_core::api::ApiResponseTrait for PatchMailGroupResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}
