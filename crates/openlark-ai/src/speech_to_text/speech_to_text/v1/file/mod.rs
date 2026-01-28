//! 文件识别
//!
//! 通过音频文件进行语音识别。
//!
//! docPath: https://open.feishu.cn/document/speech_to_text-v1/file_recognize

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::SPEECH_TO_TEXT_V1_FILE_RECOGNIZE;

/// 文件识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 音频编码格式，支持：mp3、wav、ogg、speex、m4a、aac
    pub audio_format: AudioFormat,
    /// 音频语言，支持：zh-CN、en-US
    pub language: String,
}

impl FileRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
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

/// 文件识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FileRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for FileRecognizeResponse {}

/// 文件识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeResult {
    /// 识别出的文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// 文件识别请求
#[derive(Debug, Clone)]
pub struct FileRecognizeRequest {
    config: Config,
}

impl FileRecognizeRequest {
    /// 创建新的文件识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行文件识别请求
    pub async fn execute(self, body: FileRecognizeBody) -> SDKResult<FileRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行文件识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: FileRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<FileRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<FileRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_FILE_RECOGNIZE)
                .body(serialize_params(&body, "文件识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "文件识别")
    }
}

/// 文件识别请求构建器
#[derive(Debug, Clone)]
pub struct FileRecognizeRequestBuilder {
    request: FileRecognizeRequest,
    file_token: Option<String>,
    audio_format: Option<AudioFormat>,
    language: Option<String>,
}

impl FileRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: FileRecognizeRequest::new(config),
            file_token: None,
            audio_format: None,
            language: None,
        }
    }

    /// 设置文件 token
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
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
    pub fn body(self) -> FileRecognizeBody {
        FileRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            audio_format: self.audio_format.unwrap_or(AudioFormat::Mp3),
            language: self.language.unwrap_or_default(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FileRecognizeResponse> {
        let body = self.body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FileRecognizeResponse> {
        let body = self.body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行文件识别
///
/// docPath: https://open.feishu.cn/document/speech_to_text-v1/file_recognize
pub async fn file_recognize(
    config: &Config,
    body: FileRecognizeBody,
) -> SDKResult<FileRecognizeResponse> {
    file_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行文件识别（支持自定义选项）
pub async fn file_recognize_with_options(
    config: &Config,
    body: FileRecognizeBody,
    option: RequestOption,
) -> SDKResult<FileRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<FileRecognizeResponse> =
        ApiRequest::post(SPEECH_TO_TEXT_V1_FILE_RECOGNIZE)
            .body(serialize_params(&body, "文件识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "文件识别")
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
        let builder = FileRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.audio_format.is_none());
        assert!(builder.language.is_none());
    }

    #[test]
    fn test_builder_file_token() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = FileRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_123");

        assert_eq!(builder.file_token, Some("file_token_123".to_string()));
    }

    #[test]
    fn test_builder_audio_format() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = FileRecognizeRequestBuilder::new(config.clone())
            .audio_format(AudioFormat::Wav);

        assert_eq!(builder.audio_format, Some(AudioFormat::Wav));
    }

    #[test]
    fn test_builder_language() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = FileRecognizeRequestBuilder::new(config.clone())
            .language("zh-CN");

        assert_eq!(builder.language, Some("zh-CN".to_string()));
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = FileRecognizeBody {
            file_token: "".to_string(),
            audio_format: AudioFormat::Mp3,
            language: "zh-CN".to_string(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_empty_language() {
        let body = FileRecognizeBody {
            file_token: "valid_token".to_string(),
            audio_format: AudioFormat::Mp3,
            language: "".to_string(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = FileRecognizeBody {
            file_token: "valid_token".to_string(),
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
        let body = FileRecognizeRequestBuilder::new(config.clone())
            .file_token("token_123")
            .audio_format(AudioFormat::Mp3)
            .language("en-US")
            .body();

        assert_eq!(body.file_token, "token_123");
        assert_eq!(body.audio_format, AudioFormat::Mp3);
        assert_eq!(body.language, "en-US");
    }
}
