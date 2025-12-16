/// CCM Drive Explorer API Old V2 模块
///
/// 包含所有云盘浏览器相关的API实现

use openlark_core::config::Config;

// Import migrated modules
use super::default::v2::root_folder::meta as root_folder_meta;
use super::default::v2::folder::meta as folder_meta;
use super::default::v2::folder::_folder_token as folder_create;
use super::default::v2::folder::children as folder_children;
use super::default::v2::file::_folder_token as file_create;
use super::default::v2::file::spreadsheets::_spreadsheet_token as file_spreadsheets;
use super::default::v2::file::copy::files::_file_token as file_copy;
use super::default::v2::file::docs::_doc_token as file_docs;

/// 云盘浏览器服务
#[derive(Debug, Clone)]
pub struct CcmDriveExplorerOldV2 {
    config: Config,
}

impl CcmDriveExplorerOldV2 {
    /// 创建新的云盘浏览器服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取我的空间（根文件夹）元数据
    pub fn root_folder_meta(&self) -> root_folder_meta::GetRootFolderMetaRequest {
        root_folder_meta::GetRootFolderMetaRequest::new(self.config.clone())
    }

    /// 获取文件夹元数据
    pub fn folder_meta(&self) -> folder_meta::GetFolderMetaRequest {
        folder_meta::GetFolderMetaRequest::new(self.config.clone())
    }

    /// 新建文件
    pub fn file(&self) -> file_create::CreateFileRequest {
        file_create::CreateFileRequest::new(self.config.clone())
    }

    /// 删除Sheet
    pub fn file_spreadsheets(&self) -> file_spreadsheets::DeleteSpreadsheetRequest {
        file_spreadsheets::DeleteSpreadsheetRequest::new(self.config.clone())
    }

    /// 复制文档
    pub fn file_copy(&self) -> file_copy::CopyFileRequest {
        file_copy::CopyFileRequest::new(self.config.clone())
    }

    /// 删除Doc
    pub fn file_docs(&self) -> file_docs::DeleteDocRequest {
        file_docs::DeleteDocRequest::new(self.config.clone())
    }

    /// 获取文件夹下的文档清单
    pub fn folder_children(&self) -> folder_children::GetFolderChildrenRequest {
        folder_children::GetFolderChildrenRequest::new(self.config.clone())
    }

    /// 新建文件夹
    pub fn folder(&self) -> folder_create::CreateFolderRequest {
        folder_create::CreateFolderRequest::new(self.config.clone())
    }
}