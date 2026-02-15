//! 获取门禁照片

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
pub struct GetAccessPhotoRequest {
    config: Arc<Config>,
    record_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccessPhotoResponse {
    pub data: Option<AccessPhotoData>,
}

impl ApiResponseTrait for GetAccessPhotoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPhotoData {
    pub photo_url: String,
}

impl GetAccessPhotoRequest {
    pub fn new(config: Arc<Config>, record_id: impl Into<String>) -> Self {
        Self {
            config,
            record_id: record_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetAccessPhotoResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetAccessPhotoResponse> {
        let path = format!("/open-apis/acs/v1/access_records/{}/access_photo", self.record_id);
        let req: ApiRequest<GetAccessPhotoResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取门禁照片", "响应数据为空")
        })
    }
}
