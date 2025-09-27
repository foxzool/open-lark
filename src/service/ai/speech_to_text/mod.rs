use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::Endpoints,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::ai::models::{SpeechRecognizeRequest, SpeechRecognizeResult, StreamSpeechRequest},
};

/// 语音识别服务
pub struct SpeechToTextService {
    pub config: Config,
}

/// 语音文件识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FileRecognizeResponse {
    /// 语音识别结果
    #[serde(flatten)]
    pub speech_result: SpeechRecognizeResult,
}

impl ApiResponseTrait for FileRecognizeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 流式语音识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamRecognizeResponse {
    /// 语音识别结果
    #[serde(flatten)]
    pub speech_result: SpeechRecognizeResult,
}

impl ApiResponseTrait for StreamRecognizeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpeechToTextService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 识别语音文件
    ///
    /// 该接口用于识别语音文件中的语音内容，将语音转换为文本。
    ///
    /// # 参数
    ///
    /// - `request`: 语音识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn file_recognize(
        &self,
        request: SpeechRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FileRecognizeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::SPEECH_TO_TEXT_V1_FILE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别流式语音
    ///
    /// 该接口用于识别流式语音数据，支持实时语音识别。
    ///
    /// # 参数
    ///
    /// - `request`: 流式语音识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn stream_recognize(
        &self,
        request: StreamSpeechRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<StreamRecognizeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for SpeechToTextService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "speech_to_text"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
