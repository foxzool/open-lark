//! 删除权限组

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
pub struct DeleteRuleExternalRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRuleExternalResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteRuleExternalResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteRuleExternalRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteRuleExternalResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteRuleExternalResponse> {
        let path = format!("/open-apis/acs/v1/rule_external");
        let req: ApiRequest<DeleteRuleExternalResponse> = ApiRequest::delete(&path);

        let _resp: openlark_core::api::Response<DeleteRuleExternalResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteRuleExternalResponse { data: None })
    }
}
