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
#[derive(Clone, Debug)]
pub struct DocxV1Service {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}

impl DocxV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

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
