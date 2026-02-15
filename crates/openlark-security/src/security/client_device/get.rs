//! 获取客户端设备认证信息

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
pub struct GetClientDeviceRequest {
    config: Arc<Config>,
    device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetClientDeviceResponse {
    pub data: Option<ClientDeviceData>,
}

impl ApiResponseTrait for GetClientDeviceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientDeviceData {
    pub device_id: String,
    pub device_name: String,
    pub auth_status: String,
    pub last_auth_time: String,
}

impl GetClientDeviceRequest {
    pub fn new(config: Arc<Config>, device_id: impl Into<String>) -> Self {
        Self {
            config,
            device_id: device_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetClientDeviceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetClientDeviceResponse> {
        let path = format!("/open-apis/security/v1/client_devices/{}", self.device_id);
        let req: ApiRequest<GetClientDeviceResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取客户端设备认证信息", "响应数据为空")
        })
    }
}
