//! Document (DOCX) v1 API module
//!
//! This module provides the v1 version of document API functionality,
//! including core document features.

use crate::prelude::*;

/// 文档操作服务
pub mod document;

/// 块操作服务
pub mod block;

/// 评论操作服务
pub mod comment;

/// Document v1 Service
#[derive(Clone)]
pub struct DocxV1Service {
    client: std::sync::Arc<LarkClient>,
}

impl DocxV1Service {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for DocxV1Service {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docx_v1_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the DocxV1Service functionality
    }
}
