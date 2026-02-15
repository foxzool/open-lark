//! 新增设备

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
pub struct CreateDeviceRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeviceResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateDeviceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateDeviceRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<CreateDeviceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateDeviceResponse> {
        let path = format!("/open-apis/acs/v1/devices");
        let req: ApiRequest<CreateDeviceResponse> = ApiRequest::post(&path);

        let _resp: openlark_core::api::Response<CreateDeviceResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(CreateDeviceResponse { data: None })
    }
}
