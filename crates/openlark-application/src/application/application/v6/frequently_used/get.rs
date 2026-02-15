//! 获取用户自定义常用的应用

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
pub struct GetFrequentlyUsedAppsRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFrequentlyUsedAppsResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetFrequentlyUsedAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetFrequentlyUsedAppsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<GetFrequentlyUsedAppsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetFrequentlyUsedAppsResponse> {
        let path = format!("/open-apis/application/v6/applications/frequently_used");
        let req: ApiRequest<GetFrequentlyUsedAppsResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetFrequentlyUsedAppsResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetFrequentlyUsedAppsResponse { data: None })
    }
}
