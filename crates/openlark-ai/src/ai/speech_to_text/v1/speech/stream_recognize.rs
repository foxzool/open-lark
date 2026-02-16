//! 识别流式语音

use openlark_core::config::Config;

/// 识别流式语音请求
#[derive(Debug)]
pub struct StreamRecognizeRequest {
    #[allow(dead_code)]
    config: Config,
}

impl StreamRecognizeRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
