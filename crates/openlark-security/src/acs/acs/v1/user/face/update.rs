//! 上传人脸图片

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
pub struct UpdateUserFaceRequest {
    config: Arc<Config>,
    user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserFaceResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateUserFaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateUserFaceRequest {
    pub fn new(config: Arc<Config>, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UpdateUserFaceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateUserFaceResponse> {
        let path = format!("/open-apis/acs/v1/users/{}/face", self.user_id);
        let req: ApiRequest<UpdateUserFaceResponse> = ApiRequest::put(&path);

        let _resp: openlark_core::api::Response<UpdateUserFaceResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(UpdateUserFaceResponse { data: None })
    }
}
