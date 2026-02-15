//! 删除设备

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
pub struct DeleteDeviceRequest {
    config: Arc<Config>,
    device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDeviceResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteDeviceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteDeviceRequest {
    pub fn new(config: Arc<Config>, device_id: impl Into<String>) -> Self {
        Self {
            config,
            device_id: device_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteDeviceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteDeviceResponse> {
        let path = format!("/open-apis/acs/v1/devices/{}", self.device_id);
        let req: ApiRequest<DeleteDeviceResponse> = ApiRequest::delete(&path);

        let _resp: openlark_core::api::Response<DeleteDeviceResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteDeviceResponse { data: None })
    }
}
