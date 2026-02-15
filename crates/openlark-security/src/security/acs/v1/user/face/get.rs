//! 下载人脸图片

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
pub struct GetUserFaceRequest {
    config: Arc<Config>,
    user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserFaceResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetUserFaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetUserFaceRequest {
    pub fn new(config: Arc<Config>, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetUserFaceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetUserFaceResponse> {
        let path = format!("/open-apis/acs/v1/users/{}/face", self.user_id);
        let req: ApiRequest<GetUserFaceResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetUserFaceResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetUserFaceResponse { data: None })
    }
}
