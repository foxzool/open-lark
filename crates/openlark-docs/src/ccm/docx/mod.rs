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
//! Covers 18 API interfaces.

use crate::prelude::*;

/// v1版本API
pub mod v1;

/// 文档操作API
pub mod documents;

/// Document (DOCX) Service
#[derive(Clone)]
pub struct DocxService {
    client: std::sync::Arc<LarkClient>,
}

impl DocxService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for DocxService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
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