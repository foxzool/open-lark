//! 更新应用协作者

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
pub struct UpdateApplicationCollaboratorsRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApplicationCollaboratorsResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateApplicationCollaboratorsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateApplicationCollaboratorsRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UpdateApplicationCollaboratorsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateApplicationCollaboratorsResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/collaborators", self.app_id);
        let req: ApiRequest<UpdateApplicationCollaboratorsResponse> = ApiRequest::put(&path);

        let _resp: openlark_core::api::Response<UpdateApplicationCollaboratorsResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(UpdateApplicationCollaboratorsResponse { data: None })
    }
}
