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

#[derive(Debug, Default)]
pub struct UpdateWorkflowBuilder {
    api_req: ApiRequest<UpdateWorkflowRequest>,
    app_token: String,
    workflow_id: String,
}

impl UpdateWorkflowBuilder {
    pub fn new(app_token: impl ToString, workflow_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_workflow_update".to_string();
        builder.api_req.method = "PUT".to_string();
        builder.app_token = app_token.to_string();
        builder.workflow_id = workflow_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/workflows/{}",
            builder.app_token, builder.workflow_id
        );
        builder.api_req.body = Some(UpdateWorkflowRequest::default());
        builder
    }

    pub fn status(mut self, status: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.status = status.to_string();
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
