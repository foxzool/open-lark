//! doc: https://open.feishu.cn/document/docs/bitable-v1/app-workflow/list

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListWorkflowRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListWorkflowResponse {
    pub workflows: Vec<Workflow>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Workflow {
    pub workflow_id: String,
    pub status: String,
    pub title: String,
}

impl ApiResponseTrait for ListWorkflowResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct ListWorkflow {
    config: openlark_core::config::Config,
    app_token: String,
}

impl ListWorkflow {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
        }
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<ListWorkflowResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/workflows",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::get(&url);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
