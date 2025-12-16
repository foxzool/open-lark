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

#[derive(Debug, Default)]
pub struct ListWorkflowBuilder {
    api_req: ApiRequest<ListWorkflowRequest>,
    app_token: String,
}

impl ListWorkflowBuilder {
    pub fn new(app_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_workflow_list".to_string();
        builder.api_req.method = "GET".to_string();
        builder.app_token = app_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/workflows",
            builder.app_token
        );
        builder.api_req.body = None;
        builder
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
