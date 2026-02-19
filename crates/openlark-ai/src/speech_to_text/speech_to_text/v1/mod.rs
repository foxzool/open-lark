//! Speech-to-Text V1 模块
//!
//! 提供语音转文字服务的 v1 版本 API。

pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// Speech-to-Text V1 API
#[derive(Debug, Clone)]
pub struct SpeechToTextV1 {
    config: Arc<Config>,
}

impl SpeechToTextV1 {
    /// 创建新的 Speech-to-Text V1 服务
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取识别 API
    pub fn recognize(&self) -> recognize::Recognize {
        recognize::Recognize::new(self.config.clone())
    }
}
