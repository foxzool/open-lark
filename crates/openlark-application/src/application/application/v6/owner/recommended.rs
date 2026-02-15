//! 获取管理员推荐的应用

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
pub struct GetRecommendedAppsRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecommendedAppsResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetRecommendedAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetRecommendedAppsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<GetRecommendedAppsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetRecommendedAppsResponse> {
        let path = format!("/open-apis/application/v6/applications/recommended");
        let req: ApiRequest<GetRecommendedAppsResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetRecommendedAppsResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetRecommendedAppsResponse { data: None })
    }
}
