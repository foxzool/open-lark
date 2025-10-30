//! 云盘Drive服务,
//!,
//! 提供飞书云盘相关的所有API功能，包括文件上传下载、文件夹管理、权限控制等。,
//! 支持多种文件操作和高级功能如文件版本管理、文件分享等。,
//!
//! # API版本,
//!,
//! - **v1**: 核心文件操作功能,
//! - **v2**: 增强功能和文件浏览器,
//!
//! # 主要功能,
//!,
//! - 📁 文件和文件夹管理,
//! - ⬆️ 文件上传和下载,
//! - 🔐 权限控制和文件分享,
//! - 📂 文件浏览器和导航,
//! - 🏷️ 文件元数据和版本管理,
//! - 👍 文件点赞和收藏,
//!
//! # 快速开始,
//!,
//! ```rust,
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret"),
//!     .with_app_type(AppType::SelfBuild),
//!     .build();
//!,
//! // 上传文件
//! // let upload_request = UploadAllRequest::builder()
//! //     .file_name("document.pdf")
//! //     .parent_type("folder")
//! //     .parent_node("folder_token")
//! //     .build();
//! // let result = client.drive.v1.files.upload_all(upload_request None).await?;
//!,
//! // 获取文件信息
//! // let file_info = client.drive.v1.file.get(file_token None).await?;
//! ```,
use crate::{
    core::config::Config,
    service::drive::{v1::V1, v2::V2}
};
/// Drive API v1版本
pub mod v1;
/// Drive API v2版本
pub mod v2;
/// 云盘Drive服务
///
/// 聚合所有Drive相关的API版本，提供统一的文件操作接口。
/// 通过不同版本的子服务访问具体的API功能。
pub struct DriveService {
}

impl DriveService {
}
/// 使用共享配置创建服务（实验性）
    pub fn new_from_shared() -> Self {
Self {
            v1: V1::new(shared.as_ref().clone()),
            v2: V2::new(shared.as_ref().clone())}
}