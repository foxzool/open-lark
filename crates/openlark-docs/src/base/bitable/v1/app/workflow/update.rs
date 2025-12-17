//! doc: https://open.feishu.cn/document/docs/bitable-v1/app-workflow/update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateWorkflowRequest {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateWorkflowResponse {
    pub workflow: Workflow,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Workflow {
    pub workflow_id: String,
    pub status: String,
    pub title: String,
}

impl ApiResponseTrait for UpdateWorkflowResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct UpdateWorkflow {
    config: openlark_core::config::Config,
    app_token: String,
    workflow_id: String,
    req: UpdateWorkflowRequest,
}

impl UpdateWorkflow {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, workflow_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            workflow_id: workflow_id.into(),
            req: UpdateWorkflowRequest::default(),
        }
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.req.status = status.into();
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<UpdateWorkflowResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/workflows/{}",
            self.config.base_url, self.app_token, self.workflow_id
        );
        let request = ApiRequest::put(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
