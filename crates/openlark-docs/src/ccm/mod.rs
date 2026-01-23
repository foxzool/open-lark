/// 云内容管理(ccm)模块
///
/// 包含docs、docx、ccm_doc、ccm_docs、ccm_drive_explorer、ccm_drive_permission、sheets、wiki等子项目的API实现
use openlark_core::config::Config;

/// 云内容管理服务（内部实现，通过 DocsClient 访问）
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct CcmService {
    config: Config,
}

impl CcmService {
    /// 创建新的云内容管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取旧版文档服务
    pub fn ccm_doc(&self) -> crate::ccm::ccm_doc::CcmDocService {
        crate::ccm::ccm_doc::CcmDocService::new(self.config.clone())
    }

    /// 获取云文档内容管理服务
    pub fn ccm_docs(&self) -> crate::ccm::ccm_docs::CcmDocsService {
        crate::ccm::ccm_docs::CcmDocsService::new(self.config.clone())
    }

    /// 获取云盘浏览器服务
    pub fn ccm_drive_explorer(&self) -> crate::ccm::ccm_drive_explorer::CcmDriveExplorerService {
        crate::ccm::ccm_drive_explorer::CcmDriveExplorerService::new(self.config.clone())
    }

    /// 获取文档权限管理服务
    pub fn ccm_drive_permission(
        &self,
    ) -> crate::ccm::ccm_drive_permission::CcmDrivePermissionService {
        crate::ccm::ccm_drive_permission::CcmDrivePermissionService::new(self.config.clone())
    }

    /// 获取表格服务
    pub fn ccm_sheet(&self) -> crate::ccm::ccm_sheet::CcmSheetService {
        crate::ccm::ccm_sheet::CcmSheetService::new(self.config.clone())
    }

    /// 获取云文档内容服务
    pub fn docs(&self) -> crate::ccm::docs::DocsService {
        crate::ccm::docs::DocsService::new(self.config.clone())
    }

    /// 获取文档块管理服务
    pub fn docx(&self) -> crate::ccm::docx::DocxService {
        crate::ccm::docx::DocxService::new(self.config.clone())
    }

    /// 获取云盘文件管理服务
    pub fn drive(&self) -> crate::ccm::drive::DriveService {
        crate::ccm::drive::DriveService::new(self.config.clone())
    }

    /// 获取电子表格服务
    pub fn sheets(&self) -> crate::ccm::sheets::SheetsService {
        crate::ccm::sheets::SheetsService::new(self.config.clone())
    }

    /// 获取Wiki知识库服务
    pub fn wiki(&self) -> crate::ccm::wiki::WikiService {
        crate::ccm::wiki::WikiService::new(self.config.clone())
    }
}

// 导出所有子项目模块
pub mod ccm_doc;
pub mod ccm_docs;
pub mod ccm_drive_explorer;
pub mod ccm_drive_permission;
pub mod ccm_sheet;
pub mod docs;
pub mod docx;
pub mod drive;
pub mod sheets;
pub mod wiki;
// old 模块已废弃 - 暂时注释以解决编译问题
// pub mod old;
