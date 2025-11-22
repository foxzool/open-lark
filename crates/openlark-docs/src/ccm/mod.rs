//! Content Collaboration Management (CCM) module
//!
//! This module provides comprehensive document collaboration functionality
//! including docs, sheets, wiki, drive, and related content management services.

use crate::prelude::*;

// 重新启用已修复的模块
// pub mod ccm_sheet;  // 语法错误，待修复
pub mod doc; // ✅ 已修复（v2已修复，v1仍有问题）
             // pub mod docx;       // 语法错误，待修复
             // pub mod drive;      // 语法错误，待修复
             // pub mod export_tasks;  // 语法错误，待修复
pub mod models;
// pub mod sheets;    // 语法错误，待修复
// pub mod wiki;      // 语法错误，待修复

// Re-export services for convenience（暂时禁用）
// #[cfg(feature = "ccm-doc")]
// pub use doc::DocService;
// #[cfg(feature = "ccm-docx")]
// pub use docx::DocxService;
// #[cfg(feature = "ccm-drive")]
// pub use drive::DriveService;

// 暂时禁用所有服务导出，待语法错误修复
// pub use export_tasks::ExportTasksService;
// pub use ccm_sheet::CcmSheetService;
// pub use drive::ExplorerService;
// pub use drive::PermissionService;

// 暂时注释掉的条件编译导入
// #[cfg(feature = "ccm-sheets")]
// pub use sheets::SheetsService;
// #[cfg(feature = "ccm-wiki")]
// pub use wiki::WikiService;

/// 简化版 CCM Service，仅提供基本框架
/// （复杂功能暂时禁用，待语法错误修复）
#[cfg(feature = "ccm-core")]
#[derive(Clone, Debug)]
pub struct CcmService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}

#[cfg(feature = "ccm-core")]
impl CcmService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取文档服务（支持创建、查询、编辑等完整功能）
    #[cfg(feature = "ccm-doc")]
    pub fn doc_v2(&self) -> doc::v2::DocV2Service {
        doc::v2::DocV2Service::new(openlark_core::config::Config::default())
    }

    // 其他服务暂时禁用，待语法错误修复
    // /// Get access to document services
    // #[cfg(feature = "ccm-docx")]
    // pub fn docx(&self) -> DocxService {
    //     DocxService::new(self.client.clone())
    // }

    // /// Get access to drive services
    // #[cfg(feature = "ccm-drive")]
    // pub fn drive(&self) -> DriveService {
    //     DriveService::new(self.client.clone())
    // }

    // /// Get access to sheets services
    // #[cfg(any(
    //     feature = "ccm-sheets",
    //     feature = "ccm-sheets-v2",
    //     feature = "ccm-sheets-v3",
    // ))]
    // pub fn sheets(&self) -> SheetsService {
    //     SheetsService::new(self.client.clone())
    // }
}

#[cfg(feature = "ccm-core")]
impl std::ops::Deref for CcmService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccm_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the CcmService functionality
    }
}
