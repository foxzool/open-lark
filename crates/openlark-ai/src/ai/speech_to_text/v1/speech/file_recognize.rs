//! 识别语音文件

use openlark_core::{config::Config, SDKResult};

/// 识别语音文件请求
#[derive(Debug)]
pub struct FileRecognizeRequest {
    config: Config,
}

impl FileRecognizeRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
