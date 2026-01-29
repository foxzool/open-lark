/// 文件管理模块
///
/// 提供词条图片的上传和下载功能。
use openlark_core::config::Config;

pub mod download;
pub mod upload;

pub use download::DownloadFileRequest;
pub use upload::{UploadFileRequest, UploadFileResp};

/// 文件管理服务
#[derive(Debug, Clone)]
pub struct FileService {
    config: Config,
}

impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for FileService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
