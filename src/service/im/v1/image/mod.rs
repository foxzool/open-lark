use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    trait_system::executable_builder::ExecutableBuilder,
    SDKResult,
};
use crate::impl_full_service;
use async_trait::async_trait;

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
    ) -> SDKResult<CreateImageResponse> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: crate::core::endpoints::im::IM_V1_IMAGES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params: HashMap::from([("image_type", image_type.to_string())]),
            body: image_data,
            ..Default::default()
        };

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
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_DOWNLOAD_IMAGE,
                "image_key",
                image_key,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let api_resp: BaseResponse<GetImageResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 创建图片上传Builder (推荐)
    pub fn upload_builder(&self) -> ImageUploadBuilder {
        ImageUploadBuilder::new()
    }

    /// 创建图片下载Builder (推荐)
    pub fn download_builder(&self) -> ImageDownloadBuilder {
        ImageDownloadBuilder::new()
    }
}

// 接入统一 Service 抽象（IM v1 - ImageService）
impl_full_service!(ImageService, "im.image", "v1");

/// 图片上传Builder
#[derive(Default)]
pub struct ImageUploadBuilder {
    image_type: Option<String>,
    image_data: Option<Vec<u8>>,
}

impl ImageUploadBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置图片类型
    pub fn image_type(mut self, image_type: impl ToString) -> Self {
        self.image_type = Some(image_type.to_string());
        self
    }

    /// 设置图片数据
    pub fn image_data(mut self, image_data: Vec<u8>) -> Self {
        self.image_data = Some(image_data);
        self
    }

    pub fn build(self) -> (String, Vec<u8>) {
        (
            self.image_type.unwrap_or_default(),
            self.image_data.unwrap_or_default(),
        )
    }
}

#[async_trait]
impl ExecutableBuilder<ImageService, (String, Vec<u8>), CreateImageResponse>
    for ImageUploadBuilder
{
    fn build(self) -> (String, Vec<u8>) {
        self.build()
    }

    async fn execute(self, service: &ImageService) -> SDKResult<CreateImageResponse> {
        let (image_type, image_data) = self.build();
        service.create(&image_type, image_data, None).await
    }

    async fn execute_with_options(
        self,
        service: &ImageService,
        option: RequestOption,
    ) -> SDKResult<CreateImageResponse> {
        let (image_type, image_data) = self.build();
        service.create(&image_type, image_data, Some(option)).await
    }
}

/// 图片下载Builder
#[derive(Default)]
pub struct ImageDownloadBuilder {
    image_key: Option<String>,
}

impl ImageDownloadBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置图片key
    pub fn image_key(mut self, image_key: impl ToString) -> Self {
        self.image_key = Some(image_key.to_string());
        self
    }

    pub fn build(self) -> String {
        self.image_key.unwrap_or_default()
    }
}

#[async_trait]
impl ExecutableBuilder<ImageService, String, GetImageResponse> for ImageDownloadBuilder {
    fn build(self) -> String {
        self.build()
    }

    async fn execute(self, service: &ImageService) -> SDKResult<GetImageResponse> {
        let image_key = self.build();
        service.get(&image_key, None).await
    }

    async fn execute_with_options(
        self,
        service: &ImageService,
        option: RequestOption,
    ) -> SDKResult<GetImageResponse> {
        let image_key = self.build();
        service.get(&image_key, Some(option)).await
    }
}
