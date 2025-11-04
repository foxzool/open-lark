#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use crate::{
    core::{,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::Endpoints,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    service::ai::models::{FileRecognizeRequest, OcrResult}
};
/// 光学字符识别服务
pub struct OpticalCharRecognitionService {
}
    pub config: Config,
/// OCR文字识别响应
#[derive(Debug, Clone)]
pub struct OcrResponse {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl OpticalCharRecognitionService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 识别图片中的文字
    ///,
/// 该接口用于识别图片中的文字内容，返回文字及其位置信息。
    ///,
/// # 参数
    ///,
/// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
pub async fn basic_recognize(,
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OcrResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: Endpoints::OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
impl Service for OpticalCharRecognitionService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "optical_char_recognition",
fn service_version() -> &'static str {,
        "v1",
}
}}}}}}}}}