//! 流式识别
//!
//! 通过 WebSocket 进行实时流式语音识别。
//!
//! docPath: https://open.feishu.cn/document/speech_to_text-v1/stream_recognize

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE;

/// 流式识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeBody {
    /// 音频编码格式，支持：mp3、wav、ogg、speex、m4a、aac
    pub audio_format: AudioFormat,
    /// 音频语言，支持：zh-CN、en-US
    pub language: String,
    /// 是否需要中间结果，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_intermediate_result: Option<bool>,
}

impl StreamRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
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

/// 流式识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for StreamRecognizeResponse {}

/// 流式识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeResult {
    /// 识别出的文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 是否是最终结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
    /// 置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// 流式识别请求
#[derive(Debug, Clone)]
pub struct StreamRecognizeRequest {
    config: Config,
}

impl StreamRecognizeRequest {
    /// 创建新的流式识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行流式识别请求
    pub async fn execute(
        self,
        body: StreamRecognizeBody,
    ) -> SDKResult<StreamRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行流式识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: StreamRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<StreamRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<StreamRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE)
                .body(serialize_params(&body, "流式识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "流式识别")
    }
}

/// 流式识别请求构建器
#[derive(Debug, Clone)]
pub struct StreamRecognizeRequestBuilder {
    request: StreamRecognizeRequest,
    audio_format: Option<AudioFormat>,
    language: Option<String>,
    need_intermediate_result: Option<bool>,
}

impl StreamRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: StreamRecognizeRequest::new(config),
            audio_format: None,
            language: None,
            need_intermediate_result: None,
        }
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

    /// 设置是否需要中间结果
    pub fn need_intermediate_result(mut self, need: impl Into<bool>) -> Self {
        self.need_intermediate_result = Some(need.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> StreamRecognizeBody {
        StreamRecognizeBody {
            audio_format: self.audio_format.unwrap_or(AudioFormat::Mp3),
            language: self.language.unwrap_or_default(),
            need_intermediate_result: self.need_intermediate_result,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<StreamRecognizeResponse> {
        let body = self.body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<StreamRecognizeResponse> {
        let body = self.body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行流式识别
///
/// docPath: https://open.feishu.cn/document/speech_to_text-v1/stream_recognize
pub async fn stream_recognize(
    config: &Config,
    body: StreamRecognizeBody,
) -> SDKResult<StreamRecognizeResponse> {
    stream_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行流式识别（支持自定义选项）
pub async fn stream_recognize_with_options(
    config: &Config,
    body: StreamRecognizeBody,
    option: RequestOption,
) -> SDKResult<StreamRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<StreamRecognizeResponse> =
        ApiRequest::post(SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE)
            .body(serialize_params(&body, "流式识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "流式识别")
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
        let builder = StreamRecognizeRequestBuilder::new(config.clone());

        assert!(builder.audio_format.is_none());
        assert!(builder.language.is_none());
        assert!(builder.need_intermediate_result.is_none());
    }

    #[test]
    fn test_builder_audio_format() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = StreamRecognizeRequestBuilder::new(config.clone())
            .audio_format(AudioFormat::Wav);

        assert_eq!(builder.audio_format, Some(AudioFormat::Wav));
    }

    #[test]
    fn test_builder_language() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = StreamRecognizeRequestBuilder::new(config.clone())
            .language("zh-CN");

        assert_eq!(builder.language, Some("zh-CN".to_string()));
    }

    #[test]
    fn test_builder_need_intermediate_result() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = StreamRecognizeRequestBuilder::new(config.clone())
            .need_intermediate_result(true);

        assert_eq!(builder.need_intermediate_result, Some(true));
    }

    #[test]
    fn test_body_validation_empty_language() {
        let body = StreamRecognizeBody {
            audio_format: AudioFormat::Mp3,
            language: "".to_string(),
            need_intermediate_result: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = StreamRecognizeBody {
            audio_format: AudioFormat::Wav,
            language: "zh-CN".to_string(),
            need_intermediate_result: Some(true),
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
        let body = StreamRecognizeRequestBuilder::new(config.clone())
            .audio_format(AudioFormat::Mp3)
            .language("en-US")
            .need_intermediate_result(true)
            .body();

        assert_eq!(body.audio_format, AudioFormat::Mp3);
        assert_eq!(body.language, "en-US");
        assert_eq!(body.need_intermediate_result, Some(true));
    }
}
