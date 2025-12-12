//! CCM Drive Explorer 模块
//!
//! 云盘浏览器相关API实现，包含8个API：
//! - root_folder_meta: 获取我的空间（根文件夹）元数据
//! - folder_meta: 获取文件夹元数据
//! - file: 新建文件
//! - file_copy: 复制文档
//! - file_docs: 删除Doc
//! - file_spreadsheets: 删除Sheet
//! - folder_children: 获取文件夹下的文档清单
//! - folder: 新建文件夹

use openlark_core::config::Config;

/// CCM Drive Explorer 服务
#[derive(Debug, Clone)]
pub struct CcmDriveExplorerService {
    config: Config,
}

impl CcmDriveExplorerService {
    /// 创建新的CCM Drive Explorer服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取V2版本API
    pub fn v2(&self) -> CcmDriveExplorerV2 {
        CcmDriveExplorerV2::new(self.config.clone())
    }

    /// 获取旧版版本API（兼容性保留）
    pub fn old(&self) -> crate::ccm::ccm_drive_explorer::old::v2::CcmDriveExplorerOldV2 {
        crate::ccm::ccm_drive_explorer::old::v2::CcmDriveExplorerOldV2::new(self.config.clone())
    }
}

/// CCM Drive Explorer V2 API访问器
#[derive(Debug, Clone)]
pub struct CcmDriveExplorerV2 {
    config: Config,
}

impl CcmDriveExplorerV2 {
    /// 创建新的V2 API访问器实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出版本模块
pub mod old;
pub mod v2;