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
    pub data: Option<OpenapiAuditData>,
}

impl ApiResponseTrait for GetOpenapiAuditResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenapiAuditData {
    pub logs: Vec<AuditLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub log_id: String,
    pub api_name: String,
    pub timestamp: String,
}

impl GetOpenapiAuditRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<GetOpenapiAuditResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetOpenapiAuditResponse> {
        let path = "/open-apis/security/v1/openapi_audit".to_string();
        let req: ApiRequest<GetOpenapiAuditResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取OpenAPI审计日志数据", "响应数据为空")
        })
    }
}
