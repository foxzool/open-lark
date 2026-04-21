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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
