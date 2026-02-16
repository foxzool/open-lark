//! 审批设备申报

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
pub struct UpdateDeviceApplyRecordRequest {
    config: Arc<Config>,
    device_apply_record_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeviceApplyRecordResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateDeviceApplyRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateDeviceApplyRecordRequest {
    pub fn new(config: Arc<Config>, device_apply_record_id: impl Into<String>) -> Self {
        Self {
            config,
            device_apply_record_id: device_apply_record_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UpdateDeviceApplyRecordResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateDeviceApplyRecordResponse> {
        let path = format!(
            "/open-apis/security_and_compliance/v2/device_apply_records/{}",
            self.device_apply_record_id
        );
        let req: ApiRequest<UpdateDeviceApplyRecordResponse> = ApiRequest::put(&path);

        let _resp: openlark_core::api::Response<UpdateDeviceApplyRecordResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(UpdateDeviceApplyRecordResponse { data: None })
    }
}
