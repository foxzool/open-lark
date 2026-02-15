//! 获取设备信息

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
pub struct GetDeviceRequest {
    config: Arc<Config>,
    device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeviceResponse {
    pub data: Option<DeviceData>,
}

impl ApiResponseTrait for GetDeviceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceData {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub status: String,
}

impl GetDeviceRequest {
    pub fn new(config: Arc<Config>, device_id: impl Into<String>) -> Self {
        Self {
            config,
            device_id: device_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetDeviceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetDeviceResponse> {
        let path = format!("/open-apis/security/v1/devices/{}", self.device_id);
        let req: ApiRequest<GetDeviceResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取设备信息", "响应数据为空")
        })
    }
}
