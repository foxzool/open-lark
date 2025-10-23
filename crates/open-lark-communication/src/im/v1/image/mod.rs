use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
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
},
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
impl ApiResponseTrait for CreateImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
/// 下载图片响应
