//! 上传勋章图片 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct CreateBadgeImageBuilder {
    image: String,
    config: Config,
}

impl CreateBadgeImageBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            image: String::new(),
            config,
        }
    }

    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = image.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateBadgeImageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CreateBadgeImageResponse> {
        validate_required!(self.image, "图片不能为空");

        let request_body = CreateBadgeImageRequest { image: self.image };
        let api_request: ApiRequest<CreateBadgeImageResponse> =
            ApiRequest::post("/open-apis/admin/v1/badge_images")
                .body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("上传勋章图片", "响应数据为空")
        })
    }
}

#[derive(Debug, Serialize)]
struct CreateBadgeImageRequest {
    image: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateBadgeImageResponse {
    pub image_id: String,
    pub image_url: String,
}

impl ApiResponseTrait for CreateBadgeImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
