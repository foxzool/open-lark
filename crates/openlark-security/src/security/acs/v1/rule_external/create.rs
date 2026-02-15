//! 创建或更新权限组

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
pub struct CreateRuleExternalRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRuleExternalResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateRuleExternalResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateRuleExternalRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<CreateRuleExternalResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateRuleExternalResponse> {
        let path = format!("/open-apis/acs/v1/rule_external");
        let req: ApiRequest<CreateRuleExternalResponse> = ApiRequest::post(&path);

        let _resp: openlark_core::api::Response<CreateRuleExternalResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(CreateRuleExternalResponse { data: None })
    }
}
