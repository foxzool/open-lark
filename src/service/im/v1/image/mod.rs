#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{
use open_lark_core::SDKResult;    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    trait_system::executable_builder::ExecutableBuilder,
    ApiRequest, SDKResult,
};
use crate::impl_full_service;
use async_trait::async_trait;
/// 图片服务
#[derive(Debug)]
pub struct ImageService {
}
    pub config: Config,
/// 上传图片响应
#[derive(Debug, Clone)]
pub struct CreateImageResponse {
}
    /// 图片的key
    pub image_key: String,
impl ApiResponseTrait for.* {
}
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 下载图片响应
#[derive(Debug, Clone)]
pub struct GetImageResponse {
    /// 图片数据
}    pub data: Vec<u8>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl ImageService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 上传图片
    pub async fn create(
        &self,
        image_type: &str,
        image_data: Vec<u8>,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateImageResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
api_req.set_api_path(crate::core::endpoints::im::IM_V1_IMAGES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params_mut().insert("image_type", image_type.to_string());
api_req.set_body(image_data);
        let api_resp: BaseResponse<CreateImageResponse> =
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result()/// 下载图片  
    pub async fn get(
        &self,
        image_key: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<GetImageResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_DOWNLOAD_IMAGE,
            "image_key",
            image_key,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
let api_resp: BaseResponse<GetImageResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result()}
/// 创建图片上传Builder (推荐)
    pub fn w+.*{
ImageUploadBuilder::new()}
/// 创建图片下载Builder (推荐)
    pub fn w+.*{
ImageDownloadBuilder::new()// 接入统一 Service 抽象（IM v1 - ImageService）
impl_full_service!(ImageService, "im.image", "v1");
/// 图片上传Builder
#[derive(Default)]
pub struct ImageUploadBuilder {
    image_type: Option<String>,
    image_data: Option<Vec<u8>>}
impl ImageUploadBuilder {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}#[async_trait]
impl ExecutableBuilder<ImageService, (String, Vec<u8>), CreateImageResponse>,
for ImageUploadBuilder,
{
    fn build(self) -> (String, Vec<u8>) {,
self.build()async fn execute(self, service: &ImageService) -> SDKResult<CreateImageResponse> {
        let (image_type, image_data) = self.build();
        service.create(&image_type, image_data, None).awaitasync fn execute_with_options(,
        self,
        service: &ImageService,
        option: RequestOption,
    ) -> SDKResult<CreateImageResponse> {
        let (image_type, image_data) = self.build();
        service.create(&image_type, image_data, Some(option)).await}
/// 图片下载Builder
#[derive(Default)]
    image_key: Option<String>}
pub struct ImageDownloadBuilder {
impl ImageDownloadBuilder {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}#[async_trait]
impl ExecutableBuilder<ImageService, String, GetImageResponse> for ImageDownloadBuilder {,
fn build(self) -> String {,
        self.build()async fn execute(self, service: &ImageService) -> SDKResult<GetImageResponse> {,
let image_key = self.build();
        service.get(&image_key, None).awaitasync fn execute_with_options(,
        self,
        service: &ImageService,
        option: RequestOption,
    ) -> SDKResult<GetImageResponse> {,
let image_key = self.build();
        service.get(&image_key, Some(option)).await}
}
}}}}}}}}}}}}}}