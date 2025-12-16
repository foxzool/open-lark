#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 云盘浏览器 v1 API 模块
///
/// 提供云盘浏览器相关的API功能，包括文件夹管理、文件操作等。
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDriveExplorerApi, api_utils::*};
use serde_json::json;

// 重新导出所有子模块类型
pub use file::*;
pub use file_copy::*;
pub use file_docs::*;
pub use file_spreadsheets::*;
pub use folder::*;
pub use folder_children::*;
pub use folder_meta::*;
pub use root_folder_meta::*;
pub use requests::*;
pub use responses::*;

// 子模块
mod file;
mod file_copy;
mod file_docs;
mod file_spreadsheets;
mod folder;
mod folder_children;
mod folder_meta;
mod root_folder_meta;
mod requests;
mod responses;
mod response_impls;

/// 浏览器API服务
///
/// 提供云盘浏览器的完整管理功能，包括文件夹管理、文件操作等。
/// 支持多种文件类型的统一管理。
#[derive(Clone)]
pub struct ExplorerService {
    config: Config,
}

impl ExplorerService {
    /// 创建新的浏览器API服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取根目录元数据
    ///
    /// 获取用户云盘根目录的元数据信息，包括目录属性、权限等。
    ///
    /// # 参数
    /// * `request` - 获取根目录元数据请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回根目录元数据信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_drive_explorer::explorer::{ExplorerService, RootFolderMetaRequest};
    ///
    /// let service = ExplorerService::new(config);
    /// let request = RootFolderMetaRequest::new();
    ///
    /// let response = service.root_folder_meta(request, None).await?;
    /// println!("根目录名称: {}", response.name);
    /// ```
    pub async fn root_folder_meta(
        &self,
        request: root_folder_meta::RootFolderMetaRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<root_folder_meta::RootFolderMetaResponse> {
        root_folder_meta::root_folder_meta(request, &self.config, option).await
    }

    /// 获取文件夹元数据
    ///
    /// 获取指定文件夹的元数据信息，包括文件夹属性、权限等。
    ///
    /// # 参数
    /// * `request` - 获取文件夹元数据请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回文件夹元数据信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_drive_explorer::explorer::{ExplorerService, FolderMetaRequest};
    ///
    /// let service = ExplorerService::new(config);
    /// let request = FolderMetaRequest::new("folder_token");
    ///
    /// let response = service.folder_meta(request, None).await?;
    /// println!("文件夹名称: {}", response.name);
    /// ```
    pub async fn folder_meta(
        &self,
        request: FolderMetaRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<FolderMetaData> {
        folder_meta::folder_meta(request, &self.config, option).await
    }

    /// 获取文件元数据
    ///
    /// 获取指定文件的元数据信息，包括文件属性、权限等。
    ///
    /// # 参数
    /// * `request` - 获取文件元数据请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回文件元数据信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_drive_explorer::explorer::{ExplorerService, FileRequest};
    ///
    /// let service = ExplorerService::new(config);
    /// let request = FileRequest::new("file_token");
    ///
    /// let response = service.file(request, None).await?;
    /// println!("文件名称: {}", response.name);
    /// ```
    pub async fn file(
        &self,
        request: FileRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<FileData> {
        file::file(request, &self.config, option).await
    }

    /// 复制文件
    ///
    /// 复制指定文件到目标位置，支持跨文件夹复制。
    ///
    /// # 参数
    /// * `request` - 复制文件请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回复制后的文件信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_drive_explorer::explorer::{ExplorerService, FileCopyRequest};
    ///
    /// let service = ExplorerService::new(config);
    /// let request = FileCopyRequest::new("file_token", "target_folder_token")
    ///     .name("新文件名");
    ///
    /// let response = service.file_copy(request, None).await?;
    /// println!("复制后的文件token: {}", response.file_token);
    /// ```
    pub async fn file_copy(
        &self,
        request: FileCopyRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<FileCopyData> {
        file_copy::file_copy(request, &self.config, option).await
    }

    /// 获取文档文件信息
    ///
    /// 获取指定文档文件的详细信息，包括内容、版本等。
    ///
    /// # 参数
    /// * `request` - 获取文档文件信息请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回文档文件信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_drive_explorer::explorer::{ExplorerService, FileDocsRequest};
    ///
    /// let service = ExplorerService::new(config);
    /// let request = FileDocsRequest::new("file_token");
    ///
    /// let response = service.file_docs(request, None).await?;
    /// println!("文档标题: {}", response.title);
    /// ```
    pub async fn file_docs(
        &self,
        request: FileDocsRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<FileDocsData> {
        file_docs::file_docs(request, &self.config, option).await
    }

    /// 获取表格文件信息
    ///
    /// 获取指定表格文件的详细信息，包括工作表、单元格等。
    ///
    /// # 参数
    /// * `request` - 获取表格文件信息请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回表格文件信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_drive_explorer::explorer::{ExplorerService, FileSpreadsheetsRequest};
    ///
    /// let service = ExplorerService::new(config);
    /// let request = FileSpreadsheetsRequest::new("file_token");
    ///
    /// let response = service.file_spreadsheets(request, None).await?;
    /// println!("表格标题: {}", response.title);
    /// ```
    pub async fn file_spreadsheets(
        &self,
        request: FileSpreadsheetsRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<FileSpreadsheetsData> {
        file_spreadsheets::file_spreadsheets(request, &self.config, option).await
    }

    /// 获取文件夹子内容
    ///
    /// 获取指定文件夹下的子文件和子文件夹列表。
    ///
    /// # 参数
    /// * `request` - 获取文件夹子内容请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回文件夹子内容列表，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_drive_explorer::explorer::{ExplorerService, FolderChildrenRequest};
    ///
    /// let service = ExplorerService::new(config);
    /// let request = FolderChildrenRequest::new("folder_token")
    ///     .page_size(50);
    ///
    /// let response = service.folder_children(request, None).await?;
    /// println!("找到{}个子项目", response.items.len());
    /// ```
    pub async fn folder_children(
        &self,
        request: FolderChildrenRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<FolderChildrenData> {
        folder_children::folder_children(request, &self.config, option).await
    }

    /// 创建文件夹
    ///
    /// 在指定位置创建新的文件夹。
    ///
    /// # 参数
    /// * `request` - 创建文件夹请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回创建的文件夹信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_drive_explorer::explorer::{ExplorerService, FolderRequest};
    ///
    /// let service = ExplorerService::new(config);
    /// let request = FolderRequest::new("parent_folder_token", "新文件夹");
    ///
    /// let response = service.folder(request, None).await?;
    /// println!("创建的文件夹token: {}", response.folder_token);
    /// ```
    pub async fn folder(
        &self,
        request: FolderRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<FolderData> {
        folder::folder(request, &self.config, option).await
    }
}

impl openlark_core::trait_system::service::Service for ExplorerService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "explorer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::service::Service;

    fn create_test_service() -> ExplorerService {
        let config = openlark_core::config::Config::builder().app_id("test_app_id").app_secret("test_app_secret").build();
        ExplorerService::new(config)
    }

    #[test]
    fn test_explorer_service_creation() {
        let config = openlark_core::config::Config::builder().app_id("test_app_id").app_secret("test_app_secret").build();
        let service = ExplorerService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_explorer_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(
            service.config().app_secret(),
            cloned_service.config().app_secret()
        );
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务方法
        let _root_folder_request = RootFolderMetaRequest::new();
        let _folder_meta_request = FolderMetaRequest::new("folder_token");
        let _file_request = FileRequest::new("file_token");
        let _file_copy_request = FileCopyRequest::new("file_token", "target_folder_token");
        let _file_docs_request = FileDocsRequest::new("file_token");
        let _file_spreadsheets_request = FileSpreadsheetsRequest::new("file_token");
        let _folder_children_request = FolderChildrenRequest::new("folder_token");
        let _folder_request = FolderRequest::new("parent_folder_token", "新文件夹");

        // 如果编译通过，说明模块结构正确
        assert!(true);
    }
}
