//! ccm/docx v1版本API
//!
//! 按照bizTag/project/version/resource/name.rs模式组织
//! 包含chat公告和document操作的相关API

use crate::prelude::*;

/// 聊天公告模块
pub mod chat;

/// 文档操作服务
pub mod document;


/// Document Service
pub struct DocxService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}

impl DocxService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// 导出chat模块
pub use chat::*;

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
