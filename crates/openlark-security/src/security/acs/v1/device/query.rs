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
pub struct QueryDeviceRequest {
    config: Arc<Config>,
    device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDeviceResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for QueryDeviceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryDeviceRequest {
    pub fn new(config: Arc<Config>, device_id: impl Into<String>) -> Self {
        Self {
            config,
            device_id: device_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<QueryDeviceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<QueryDeviceResponse> {
        let path = format!("/open-apis/acs/v1/devices/{}/query", self.device_id);
        let req: ApiRequest<QueryDeviceResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<QueryDeviceResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(QueryDeviceResponse { data: None })
    }
}
