/// 文件管理模块
///
/// 提供词条图片的上传和下载功能。
use openlark_core::config::Config;

// 导出具体的API实现
pub mod download;
pub mod upload;

// 重新导出API函数
pub use download::*;
pub use upload::*;

/// 文件管理服务
#[derive(Debug, Clone)]
pub struct FileService {
    config: Config,
}

impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for FileService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
