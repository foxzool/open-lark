//! 获取OpenAPI审计日志数据

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
pub struct GetOpenapiAuditRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenapiAuditResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetOpenapiAuditResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetOpenapiAuditRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<GetOpenapiAuditResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetOpenapiAuditResponse> {
        let path = format!("/open-apis/acs/v1/openapi_audit");
        let req: ApiRequest<GetOpenapiAuditResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetOpenapiAuditResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetOpenapiAuditResponse { data: None })
    }
}
