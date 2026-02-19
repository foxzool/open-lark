//! Speech-to-Text 模块
//!
//! 提供语音转文字服务，支持文件识别、流式识别和通用语音识别。
//!
//! ## 主要功能
//!
//! - **文件识别**: 上传整段语音文件进行一次性识别，适合 60 秒以内音频
//! - **流式识别**: 将音频文件分片传入模型，能够实时返回数据
//! - **通用语音识别**: 支持多种音频格式和识别配置
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_ai::speech_to_text::speech_to_text::v1::recognize::file_recognize::FileRecognizeBody;
//! use openlark_ai::prelude::Config;
//!
//! let config = Config::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build();
//!
//! // 文件识别
//! let body = FileRecognizeBody {
//!     file_token: "your_file_token".to_string(),
//!     is_async: None,
//!     language: None,
//!     format: None,
//! };
//! // let response = file_recognize(&config, body).await;
//! ```

pub mod speech_to_text;

use openlark_core::config::Config;
use std::sync::Arc;

/// Speech-to-Text 服务
#[derive(Debug, Clone)]
pub struct SpeechToTextService {
    config: Arc<Config>,
}

impl SpeechToTextService {
    /// 创建新的 Speech-to-Text 服务
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 V1 版本 API
    pub fn v1(&self) -> speech_to_text::v1::SpeechToTextV1 {
        speech_to_text::v1::SpeechToTextV1::new(self.config.clone())
    }
}
