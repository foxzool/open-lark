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
pub mod import_task;
pub mod export_task;
pub mod file_version;

// 重新导出所有服务类型
pub use files::*;
pub use folder::*;
pub use import_task::*;
pub use export_task::*;
pub use file_version::*;

use crate::core::config::Config;

/// Drive服务 v1版本
///
/// 云盘服务的统一入口，提供文件和文件夹的完整管理功能。
/// 支持企业级的文件存储、权限控制、版本管理等高级特性。
#[derive(Debug, Clone)]
pub struct DriveServiceV1 {
    config: Config,
    transport: std::sync::Arc<dyn crate::core::transport::Transport>,
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
    /// - `transport`: 传输层实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::drive::v1::DriveServiceV1;
    /// use std::sync::Arc;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let transport = Arc::new(MockTransport::new());
    /// let drive_service = DriveServiceV1::new(config, transport);
    /// ```
    pub fn new(config: Config, transport: std::sync::Arc<dyn crate::core::transport::Transport>) -> Self {
        Self {
            config: config.clone(),
            transport: transport.clone(),
            files: FilesService::new(config.clone()),
            folder: FolderService::new(config),
        }
    }

    /// 创建导入任务构建器
    ///
    /// 创建一个导入任务的构建器，支持将外部文件导入到飞书云文档
    ///
    /// # 参数
    /// * `request` - 导入任务请求，包含文件类型、名称、URL等信息
    ///
    /// # 返回
    /// 返回导入任务构建器，可用于执行导入操作
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::drive::v1::import_task::{CreateImportTaskRequest, CreateImportTaskResponse};
    ///
    /// async fn create_import_task_example(
    ///     service: std::sync::Arc<DriveServiceV1>,
    /// ) -> Result<CreateImportTaskResponse, Box<dyn std::error::Error>> {
    ///     let request = CreateImportTaskRequest::builder()
    ///         .file_type("docx")
    ///         .file_name("报告.docx")
    ///         .file_url("https://example.com/file.docx")
    ///         .parent_folder_token("folder_token")
    ///         .overwrite(true)
    ///         .build()?;
    ///
    ///     let response = service
    ///         .create_import_task_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("导入任务创建成功，任务ID: {}", response.task_id());
    ///     Ok(response)
    /// }
    /// ```
    pub fn create_import_task_builder(&self, request: CreateImportTaskRequest) -> CreateImportTaskBuilder {
        CreateImportTaskBuilder::new(std::sync::Arc::new(self.clone()), request)
    }

    /// 创建导出任务构建器
    ///
    /// 创建一个导出任务的构建器，支持将飞书云文档导出为各种格式
    ///
    /// # 参数
    /// * `request` - 导出任务请求，包含文件令牌、导出格式等信息
    ///
    /// # 返回
    /// 返回导出任务构建器，可用于执行导出操作
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::drive::v1::export_task::{CreateExportTaskRequest, CreateExportTaskResponse};
    ///
    /// async fn create_export_task_example(
    ///     service: std::sync::Arc<DriveServiceV1>,
    /// ) -> Result<CreateExportTaskResponse, Box<dyn std::error::Error>> {
    ///     let request = CreateExportTaskRequest::builder()
    ///         .file_token("file_token_123")
    ///         .export_format("pdf")
    ///         .file_type("doc")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .create_export_task_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("导出任务创建成功，任务ID: {}", response.task_id());
    ///     Ok(response)
    /// }
    /// ```
    pub fn create_export_task_builder(&self, request: CreateExportTaskRequest) -> CreateExportTaskBuilder {
        CreateExportTaskBuilder::new(std::sync::Arc::new(self.clone()), request)
    }

    /// 获取文档版本信息构建器
    ///
    /// 创建一个获取文档版本信息的构建器，支持获取文件特定版本的详细信息
    ///
    /// # 参数
    /// * `request` - 获取文档版本信息请求，包含文件令牌和版本ID
    ///
    /// # 返回
    /// 返回获取文档版本信息构建器，可用于执行查询操作
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::drive::v1::file_version::{GetFileVersionRequest, GetFileVersionResponse};
    ///
    /// async fn get_file_version_example(
    ///     service: std::sync::Arc<DriveServiceV1>,
    /// ) -> Result<GetFileVersionResponse, Box<dyn std::error::Error>> {
    ///     let request = GetFileVersionRequest::builder()
    ///         .file_token("file_token_123")
    ///         .version_id("version_456")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .get_file_version_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("获取版本信息成功，版本号: {}", response.version_number());
    ///     Ok(response)
    /// }
    /// ```
    pub fn get_file_version_builder(&self, request: GetFileVersionRequest) -> GetFileVersionBuilder {
        GetFileVersionBuilder::new(std::sync::Arc::new(self.clone()), request)
    }

    /// 删除文档版本构建器
    ///
    /// 创建一个删除文档版本的构建器，支持删除文件的指定版本。
    /// ⚠️ 删除操作不可逆，请谨慎使用。
    ///
    /// # 参数
    /// * `request` - 删除文档版本请求，包含文件令牌、版本ID和删除确认
    ///
    /// # 返回
    /// 返回删除文档版本构建器，可用于执行删除操作
    ///
    /// # 安全措施
    /// * 必须显式确认删除操作（confirm=true）
    /// * 验证文件令牌和版本ID的有效性
    /// * 删除版本不会影响文件的其他版本
    /// * 如果删除的是当前版本，系统会自动将最新版本设为当前版本
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::drive::v1::file_version::{DeleteFileVersionRequest, DeleteFileVersionResponse};
    ///
    /// async fn delete_file_version_example(
    ///     service: std::sync::Arc<DriveServiceV1>,
    /// ) -> Result<DeleteFileVersionResponse, Box<dyn std::error::Error>> {
    ///     let request = DeleteFileVersionRequest::builder()
    ///         .file_token("file_token_123")
    ///         .version_id("version_456")
    ///         .confirm(true)  // 必须显式确认删除
    ///         .build()?;
    ///
    ///     let response = service
    ///         .delete_file_version_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("版本删除成功，删除时间: {}", response.deleted_at());
    ///     Ok(response)
    /// }
    /// ```
    pub fn delete_file_version_builder(&self, request: DeleteFileVersionRequest) -> DeleteFileVersionBuilder {
        DeleteFileVersionBuilder::new(std::sync::Arc::new(self.clone()), request)
    }
}

impl crate::core::service_trait::Service for DriveServiceV1 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn transport(&self) -> &dyn crate::core::transport::Transport {
        self.transport.as_ref()
    }
}

// DriveServiceV1不提供Default实现，因为它需要transport参数

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::transport::MockTransport;

    fn create_test_service() -> DriveServiceV1 {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = std::sync::Arc::new(MockTransport::new());
        DriveServiceV1::new(config, transport)
    }

    #[test]
    fn test_drive_service_creation() {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = std::sync::Arc::new(MockTransport::new());
        let service = DriveServiceV1::new(config.clone(), transport);

        assert_eq!(service.config().app_id(), config.app_id());
        assert_eq!(service.config().app_secret(), config.app_secret());
    }

    #[test]
    fn test_import_task_builder_creation() {
        let service = create_test_service();
        let request = CreateImportTaskRequest::builder()
            .file_type("docx")
            .file_name("test.docx")
            .file_url("https://example.com/test.docx")
            .build()
            .unwrap();

        let builder = service.create_import_task_builder(request);
        // 验证构建器创建成功
        assert_eq!(builder.request.file_type, "docx");
        assert_eq!(builder.request.file_name, "test.docx");
        assert_eq!(builder.request.file_url, "https://example.com/test.docx");
    }

    #[test]
    fn test_export_task_builder_creation() {
        let service = create_test_service();
        let request = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("pdf")
            .file_type("doc")
            .build()
            .unwrap();

        let builder = service.create_export_task_builder(request);
        // 验证构建器创建成功
        assert_eq!(builder.request.file_token, "file_token_123");
        assert_eq!(builder.request.export_format, "pdf");
        assert_eq!(builder.request.file_type, "doc");
    }

    #[test]
    fn test_get_file_version_builder_creation() {
        let service = create_test_service();
        let request = GetFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("version_456")
            .build()
            .unwrap();

        let builder = service.get_file_version_builder(request);
        // 验证构建器创建成功
        assert_eq!(builder.request.file_token, "file_token_123");
        assert_eq!(builder.request.version_id, "version_456");
    }

    #[test]
    fn test_delete_file_version_builder_creation() {
        let service = create_test_service();
        let request = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("version_456")
            .confirm(true)  // 必须确认删除
            .build()
            .unwrap();

        let builder = service.delete_file_version_builder(request);
        // 验证构建器创建成功
        assert_eq!(builder.request.file_token, "file_token_123");
        assert_eq!(builder.request.version_id, "version_456");
        assert_eq!(builder.request.confirm, true);
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");

        let transport_ref = service.transport();
        // Transport trait测试需要具体的mock实现
        assert!(!std::ptr::eq(transport_ref, std::ptr::null()));
    }

    #[test]
    fn test_drive_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(service.config().app_secret(), cloned_service.config().app_secret());
    }
}