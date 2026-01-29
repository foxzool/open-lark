/// 文件管理模块
///
/// 提供词条图片的上传和下载功能。
pub mod download;
pub mod upload;

pub use download::DownloadFileRequest;
pub use upload::{UploadFileRequest, UploadFileResp};
