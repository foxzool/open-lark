/// ccm/docx模块 - 文档块内容管理
///
/// 按照bizTag/project/version/resource/name.rs模式组织
/// 包含chat公告和document操作的相关API
use openlark_core::config::Config;

/// 公共类型定义
pub mod common_types;

/// 数据模型定义
pub mod models;

/// docx API 数据模型
pub mod models_docx;

// 重新导出常用类型，方便其他模块使用
pub use common_types::*;

/// API服务实现 (临时注释以修复编译问题)
// pub mod services;

/// v1版本API
pub mod v1;

/// 文档操作API
pub mod documents;

/// Document (DOCX) Service
pub struct DocxService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
    // ccm_docs API服务 (临时注释以修复编译问题)
    // #[cfg(feature = "ccm-docx")]
    // pub ccm_docs: services::CcmDocsService,
    // docx API服务 (临时注释以修复编译问题)
    // #[cfg(feature = "ccm-docx")]
    // pub docx: services::DocxService,
}

impl DocxService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            // #[cfg(feature = "ccm-docx")]
            // ccm_docs: services::CcmDocsService::new(config.clone()),
            // #[cfg(feature = "ccm-docx")]
            // docx: services::DocxService::new(config.clone()),
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
