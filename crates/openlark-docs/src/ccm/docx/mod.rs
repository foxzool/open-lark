/// ccm/docx模块 - 文档块内容管理
///
/// 按照bizTag/project/version/resource/name.rs模式组织
/// 包含chat公告和document操作的相关API
use openlark_core::config::Config;

/// 数据模型定义（模型，不算 API）
pub mod models;

/// Docx 模块公共类型（模型，不算 API）
pub use models::common_types;

/// Docx API 数据模型（模型，不算 API）
pub use models::models_docx;

// 重新导出常用类型，方便其他模块使用
pub use common_types::*;

// API服务实现
// pub mod services;

/// v1版本API
pub mod v1;

/// 文档操作API
pub mod documents;

/// Document (DOCX) Service
#[derive(Debug, Clone)]
pub struct DocxService {
    config: Config,
}

impl DocxService {
    /// 创建新的文档块管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取V1版本API
    pub fn v1(&self) -> crate::ccm::docx::v1::DocxService {
        crate::ccm::docx::v1::DocxService::new(self.config.clone())
    }

    /// 获取文档操作API
    pub fn documents(&self) -> crate::ccm::docx::documents::DocumentsService {
        crate::ccm::docx::documents::DocumentsService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::DocxService;
    use openlark_core::config::Config;

    #[test]
    fn test_docx_service_creation() {
        let service = DocxService::new(Config::default());
        assert!(!service.config().base_url.is_empty());
    }
}
