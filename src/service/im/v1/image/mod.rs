use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 图片服务
pub struct ImageService {
    pub config: Config,
}

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
    ) -> SDKResult<BaseResponse<CreateImageResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/im/v1/images".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params: HashMap::from([("image_type".to_string(), image_type.to_string())]),
            body: image_data,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 下载图片  
    pub async fn get(
        &self,
        image_key: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetImageResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/im/v1/images/{}", image_key),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
