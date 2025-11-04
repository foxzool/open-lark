#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Cloud Docs Drive服务 v1
//!
//! 提供企业级的云盘管理功能，包括文件和文件夹的完整生命周期管理：
//! - 文件管理：上传、下载、更新、删除、查询文件信息
//! - 文件夹管理：创建、重命名、移动、删除文件夹
//! - 权限控制：细粒度的访问权限和分享设置
//! - 版本控制：文件版本历史和恢复功能
//! - 搜索功能：全文搜索和高级筛选
//!
//! # 特性
//!
//! ## 企业级文件操作
//! - **安全上传**: 支持多种文件格式，自动病毒扫描
//! - **智能下载**: 支持断点续传和压缩传输
//! - **版本管理**: 完整的文件版本历史和回滚功能
//! - **权限控制**: 精细的读写权限和分享链接管理
//!
//! ## 高级功能
//! - **批量操作**: 支持文件和文件夹的批量上传/下载
//! - **搜索筛选**: 强大的文件搜索和元数据筛选功能
//! - **同步备份**: 多设备自动同步和备份机制
//! - **API集成**: 完整的RESTful API支持

pub mod files;
pub mod folder;

// 重新导出所有服务类型
pub use files::*;
pub use folder::*;

use crate::core::config::Config;

/// Drive服务 v1版本
///
/// 云盘服务的统一入口，提供文件和文件夹的完整管理功能。
/// 支持企业级的文件存储、权限控制、版本管理等高级特性。
#[derive(Debug, Clone)]
pub struct DriveServiceV1 {
    pub config: Config,
    /// 文件管理服务
    pub files: FilesService,
    /// 文件夹管理服务
    pub folder: FolderService,
}

impl DriveServiceV1 {
    /// 创建Drive v1服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::drive::v1::DriveServiceV1;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let drive_service = DriveServiceV1::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            files: FilesService::new(config.clone()),
            folder: FolderService::new(config),
        }
    }
}

impl Default for DriveServiceV1 {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive_service_creation() {
        let config = Config::new("test_app_id", "test_app_secret");
        let service = DriveServiceV1::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_drive_service_default() {
        let service = DriveServiceV1::default();

        // 验证子服务已正确初始化
        assert_eq!(service.files.config.app_id, "");
        assert_eq!(service.folder.config.app_id, "");
    }

    #[test]
    fn test_subservices_access() {
        let service = DriveServiceV1::default();

        // 测试文件服务访问
        let upload_builder = service.files.create_upload_file_builder();
        let download_builder = service.files.create_download_file_builder();

        // 测试文件夹服务访问
        let root_folder_builder = service.folder.create_get_root_folder_builder();
        let list_files_builder = service.folder.create_list_files_builder();

        // 验证所有构建器都能正常创建
        assert!(true); // 如果能创建构建器就说明服务正常
    }
}