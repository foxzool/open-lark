//! 语音识别模块
//!
//! 提供语音识别相关的 API，包括文件识别、流式识别和通用语音识别。

pub mod file_recognize;
pub mod speech_recognize;
pub mod stream_recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 语音识别 API
#[derive(Debug, Clone)]
pub struct Recognize {
    config: Arc<Config>,
}

impl Recognize {
    /// 创建新的识别服务
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 文件识别请求
    pub fn file_recognize(&self) -> file_recognize::FileRecognizeRequest {
        file_recognize::FileRecognizeRequest::new((*self.config).clone())
    }

    /// 流式识别请求
    pub fn stream_recognize(&self) -> stream_recognize::StreamRecognizeRequest {
        stream_recognize::StreamRecognizeRequest::new((*self.config).clone())
    }

    /// 通用语音识别请求
    pub fn speech_recognize(&self) -> speech_recognize::SpeechRecognizeRequest {
        speech_recognize::SpeechRecognizeRequest::new((*self.config).clone())
    }
}
