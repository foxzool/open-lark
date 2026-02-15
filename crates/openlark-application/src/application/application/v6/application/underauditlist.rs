//! 查看待审核的应用列表

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
pub struct GetApplicationUnderauditlistRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationUnderauditlistResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationUnderauditlistResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationUnderauditlistRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<GetApplicationUnderauditlistResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationUnderauditlistResponse> {
        let path = format!("/open-apis/application/v6/applications/underauditlist");
        let req: ApiRequest<GetApplicationUnderauditlistResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetApplicationUnderauditlistResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetApplicationUnderauditlistResponse { data: None })
    }
}
