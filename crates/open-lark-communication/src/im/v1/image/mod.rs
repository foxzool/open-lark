use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use open_lark_core::standard_response::StandardResponse;
use open_lark_core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 图片服务
pub struct ImageService {
    pub config: Config,
}

// 接入统一 Service 抽象（IM v1 - ImageService）
// impl_full_service!(ImageService, "im.image", "v1");

/// 上传图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageResponse {
    /// 图片的key
    pub image_key: String,
}

impl ApiResponseTrait for CreateImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImageResponse {
    /// 图片数据
    pub data: Vec<u8>,
}

impl ApiResponseTrait for GetImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ImageService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传图片
    pub async fn create(
        &self,
        image_type: &str,
        image_data: Vec<u8>,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateImageResponse> {
        let mut query_params = HashMap::new();
        query_params.insert("image_type", image_type.to_string());

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(open_lark_core::endpoints::im::IM_V1_IMAGES.to_string());
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = query_params;
        api_req.body = image_data;

        let api_resp: BaseResponse<CreateImageResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 下载图片
    pub async fn get(
        &self,
        image_key: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<GetImageResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::endpoints::im::IM_V1_DOWNLOAD_IMAGE,
            "image_key",
            image_key,
        ));
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        let api_resp: BaseResponse<GetImageResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use open_lark_core::config::Config;
    #[test]
    fn test_create_image_response_format() {
        assert_eq!(CreateImageResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_image_response_format() {
        assert_eq!(GetImageResponse::data_format(), ResponseFormat::Data);
    }
}
