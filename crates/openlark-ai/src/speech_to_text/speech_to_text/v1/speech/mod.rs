//! 语音识别
//!
//! 通用语音识别服务。
//!
//! docPath: https://open.feishu.cn/document/speech_to_text-v1/speech_recognize

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE;

/// 语音识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechRecognizeBody {
    /// 音频数据的 Base64 编码
    pub audio_data: String,
    /// 音频编码格式，支持：mp3、wav、ogg、speex、m4a、aac
    pub audio_format: AudioFormat,
    /// 音频语言，支持：zh-CN、en-US
    pub language: String,
}

impl SpeechRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.audio_data.trim().is_empty() {
            return Err("audio_data 不能为空".to_string());
        }
        if self.language.trim().is_empty() {
            return Err("language 不能为空".to_string());
        }
        Ok(())
    }
}

/// 音频编码格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    /// MP3 格式
    Mp3,
    /// WAV 格式
    Wav,
    /// OGG 格式
    Ogg,
    /// Speex 格式
    Speex,
    /// M4A 格式
    M4a,
    /// AAC 格式
    Aac,
}

/// 语音识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SpeechRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for SpeechRecognizeResponse {}

/// 语音识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechRecognizeResult {
    /// 识别出的文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// 语音识别请求
#[derive(Debug, Clone)]
pub struct SpeechRecognizeRequest {
    config: Config,
}

impl SpeechRecognizeRequest {
    /// 创建新的语音识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行语音识别请求
    pub async fn execute(
        self,
        body: SpeechRecognizeBody,
    ) -> SDKResult<SpeechRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行语音识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: SpeechRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<SpeechRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<SpeechRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE)
                .body(serialize_params(&body, "语音识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "语音识别")
    }
}

/// 语音识别请求构建器
#[derive(Debug, Clone)]
pub struct SpeechRecognizeRequestBuilder {
    request: SpeechRecognizeRequest,
    audio_data: Option<String>,
    audio_format: Option<AudioFormat>,
    language: Option<String>,
}

impl SpeechRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: SpeechRecognizeRequest::new(config),
            audio_data: None,
            audio_format: None,
            language: None,
        }
    }

    /// 设置音频数据（Base64 编码）
    pub fn audio_data(mut self, audio_data: impl Into<String>) -> Self {
        self.audio_data = Some(audio_data.into());
        self
    }

    /// 设置音频格式
    pub fn audio_format(mut self, audio_format: AudioFormat) -> Self {
        self.audio_format = Some(audio_format);
        self
    }

    /// 设置语言
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> SpeechRecognizeBody {
        SpeechRecognizeBody {
            audio_data: self.audio_data.unwrap_or_default(),
            audio_format: self.audio_format.unwrap_or(AudioFormat::Mp3),
            language: self.language.unwrap_or_default(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SpeechRecognizeResponse> {
        let body = self.body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SpeechRecognizeResponse> {
        let body = self.body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行语音识别
///
/// docPath: https://open.feishu.cn/document/speech_to_text-v1/speech_recognize
pub async fn speech_recognize(
    config: &Config,
    body: SpeechRecognizeBody,
) -> SDKResult<SpeechRecognizeResponse> {
    speech_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行语音识别（支持自定义选项）
pub async fn speech_recognize_with_options(
    config: &Config,
    body: SpeechRecognizeBody,
    option: RequestOption,
) -> SDKResult<SpeechRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<SpeechRecognizeResponse> =
        ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE)
            .body(serialize_params(&body, "语音识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "语音识别")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = SpeechRecognizeRequestBuilder::new(config.clone());

        assert!(builder.audio_data.is_none());
        assert!(builder.audio_format.is_none());
        assert!(builder.language.is_none());
    }

    #[test]
    fn test_builder_audio_data() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = SpeechRecognizeRequestBuilder::new(config.clone())
            .audio_data("base64_encoded_audio");

        assert_eq!(builder.audio_data, Some("base64_encoded_audio".to_string()));
    }

    #[test]
    fn test_builder_audio_format() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = SpeechRecognizeRequestBuilder::new(config.clone())
            .audio_format(AudioFormat::Wav);

        assert_eq!(builder.audio_format, Some(AudioFormat::Wav));
    }

    #[test]
    fn test_builder_language() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = SpeechRecognizeRequestBuilder::new(config.clone())
            .language("zh-CN");

        assert_eq!(builder.language, Some("zh-CN".to_string()));
    }

    #[test]
    fn test_body_validation_empty_audio_data() {
        let body = SpeechRecognizeBody {
            audio_data: "".to_string(),
            audio_format: AudioFormat::Mp3,
            language: "zh-CN".to_string(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_empty_language() {
        let body = SpeechRecognizeBody {
            audio_data: "valid_data".to_string(),
            audio_format: AudioFormat::Mp3,
            language: "".to_string(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = SpeechRecognizeBody {
            audio_data: "valid_data".to_string(),
            audio_format: AudioFormat::Wav,
            language: "zh-CN".to_string(),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_from_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = SpeechRecognizeRequestBuilder::new(config.clone())
            .audio_data("data_123")
            .audio_format(AudioFormat::Mp3)
            .language("en-US")
            .body();

        assert_eq!(body.audio_data, "data_123");
        assert_eq!(body.audio_format, AudioFormat::Mp3);
        assert_eq!(body.language, "en-US");
    }
}
