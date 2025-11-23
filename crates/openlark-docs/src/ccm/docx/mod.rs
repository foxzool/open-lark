//! Document (DOCX) module for OpenLark Docs
//!
//! This module provides comprehensive document (DOCX) functionality including:
//! - Document creation, reading, updating, deletion
//! - Document block operations (text, images, tables, etc.)
//! - Document version management
//! - Document comments and replies
//! - Document import/export
//! - Document search and statistics
//!
//! ## ccm_docs API
//! - [`services::CcmDocsService`] - 云文档搜索和元数据API
//! - [`models`] - ccm_docs API 数据模型
//!
//! ## v1版本API（框架）
//! - [`v1`] - v1版本的文档操作框架
//! - [`documents`] - 文档操作API框架

use crate::prelude::*;

/// 数据模型定义
pub mod models;

/// docx API 数据模型
pub mod models_docx;

/// API服务实现
pub mod services;

/// v1版本API
pub mod v1;

/// 文档操作API
pub mod documents;

/// Document (DOCX) Service
#[derive(Clone, Debug)]
pub struct DocxService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
    /// ccm_docs API服务
    #[cfg(feature = "ccm-docx")]
    pub ccm_docs: services::CcmDocsService,
    /// docx API服务
    #[cfg(feature = "ccm-docx")]
    pub docx: services::DocxService,
}

impl DocxService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "ccm-docx")]
            ccm_docs: services::CcmDocsService::new(config.clone()),
            #[cfg(feature = "ccm-docx")]
            docx: services::DocxService::new(config.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docx_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the DocxService functionality
    }
}
