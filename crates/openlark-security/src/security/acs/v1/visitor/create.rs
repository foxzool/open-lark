//! 添加访客

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
pub struct CreateVisitorRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVisitorResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateVisitorResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateVisitorRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<CreateVisitorResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateVisitorResponse> {
        let path = format!("/open-apis/acs/v1/visitors");
        let req: ApiRequest<CreateVisitorResponse> = ApiRequest::post(&path);

        let _resp: openlark_core::api::Response<CreateVisitorResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(CreateVisitorResponse { data: None })
    }
}
