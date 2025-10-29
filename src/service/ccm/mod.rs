// ccm - 云文档管理模块
//
// 该模块提供飞书云文档相关的所有功能，包括：
// - 云空间文件管理 (drive)
// - 电子表格操作 (sheets)
// - 文档操作 (docx/doc)
// - 知识库管理 (wiki)
// - 文档权限管理 (permission)
// - 导出任务 (export_tasks)
//
// 覆盖174个API接口，是企业协作的核心功能模块

use crate::prelude::*;
use crate::service::ccm::drive::DriveService;
use crate::service::ccm::sheets::SheetsService;
use crate::service::ccm::docx::DocxService;
use crate::service::ccm::wiki::WikiService;

/// 云文档管理服务
#[cfg(feature = "ccm")]
#[derive(Debug, Clone)]
pub struct CcmService {
    /// 云空间文件管理服务
    pub drive: DriveService,
    /// 电子表格服务
    pub sheets: SheetsService,
    /// 文档服务
    pub docx: DocxService,
    /// 知识库服务
    pub wiki: WikiService,
}

#[cfg(feature = "ccm")]
impl CcmService {
    /// 创建新的云文档管理服务实例
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self {
            drive: DriveService::new(client.clone()),
            sheets: SheetsService::new(client.clone()),
            docx: DocxService::new(client.clone()),
            wiki: WikiService::new(client.clone()),
        }
    }
}

#[cfg(not(feature = "ccm"))]
pub struct CcmService;

/// 数据模型
pub mod models;

/// 各子模块
pub mod drive;
pub mod sheets;
pub mod docx;
pub mod doc;
pub mod wiki;
pub mod permission;
pub mod export_tasks;