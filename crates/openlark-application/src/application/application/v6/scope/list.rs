//! 查询租户授权状态

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
pub struct ListApplicationScopeRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListApplicationScopeResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListApplicationScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListApplicationScopeRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<ListApplicationScopeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListApplicationScopeResponse> {
        let path = format!("/open-apis/application/v6/scopes");
        let req: ApiRequest<ListApplicationScopeResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<ListApplicationScopeResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(ListApplicationScopeResponse { data: None })
    }
}
