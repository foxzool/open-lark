use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::ai::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::ai::models::{
        LanguageDetectRequest, LanguageDetectResult, TranslateRequest, TranslateResult,
    },
};

/// 机器翻译服务
pub struct TranslationService {
    pub config: Config,
}

/// 语种检测响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DetectResponse {
    /// 语种检测结果
    #[serde(flatten)]
    pub detect_result: LanguageDetectResult,
}

impl ApiResponseTrait for DetectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文本翻译响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponse {
    /// 翻译结果
    #[serde(flatten)]
    pub translate_result: TranslateResult,
}

impl ApiResponseTrait for TranslateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TranslationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 识别文本语种
    ///
    /// 该接口用于识别输入文本的语言种类。
    ///
    /// # 参数
    ///
    /// - `request`: 语种检测请求参数
    /// - `option`: 可选的请求配置
    pub async fn detect(
        &self,
        request: LanguageDetectRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DetectResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: TRANSLATION_V1_TEXT_DETECT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 翻译文本
    ///
    /// 该接口用于将文本翻译为指定的目标语言。
    ///
    /// # 参数
    ///
    /// - `request`: 文本翻译请求参数
    /// - `option`: 可选的请求配置
    pub async fn translate(
        &self,
        request: TranslateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TranslateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: TRANSLATION_V1_TEXT_TRANSLATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for TranslationService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "translation"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
