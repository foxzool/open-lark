//! Speech-to-Text 模块
//!
//! 提供语音转文字服务，包括文件识别、流式识别和语音识别。

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Speech-to-Text API 入口
#[derive(Clone)]
pub struct SpeechToText {
    config: Arc<Config>,
}

impl SpeechToText {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn v1(&self) -> v1::SpeechToTextV1 {
        v1::SpeechToTextV1::new(self.config.clone())
    }
}
