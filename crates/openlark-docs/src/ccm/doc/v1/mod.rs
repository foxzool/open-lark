//! Docs文档服务 v1
//!
//! 提供飞书文档v1版本的完整管理功能，包括：
//! - 创建和删除文档
//! - 文档信息查询和管理
//! - 文档内容读取和编辑
//! - 文档块操作和管理

pub mod document;
pub mod models;
pub mod requests;
pub mod responses;

// 重新导出所有服务类型
pub use document::*;

use openlark_core::config::Config;

/// Docs文档服务 v1版本
///
/// 提供飞书文档v1版本的统一入口，支持现代化的文档管理。
/// 包括创建、编辑、格式化、协作等企业级功能。
#[derive(Clone, Debug)]
pub struct DocsServiceV1 {
    config: Config,
    /// 文档管理服务
    pub document: DocumentService,
}

impl DocsServiceV1 {
    /// 创建Docs v1服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v1::DocsServiceV1;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = DocsServiceV1::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone()
            document: DocumentService::new(config),
        }
    }
}

impl openlark_core::trait_system::Service for DocsServiceV1 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "DocsServiceV1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::prelude::Service;

    #[test]
    fn test_docs_v1_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocsServiceV1::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_v1_service_trait_implementation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocsServiceV1::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }

    #[test]
    fn test_document_service_available() {
        let config = openlark_core::config::Config::default();
        let service = DocsServiceV1::new(config);

        // 验证document服务可用
        let document_service_str = format!("{:?}", service.document);
        assert!(!document_service_str.is_empty());
    }
}
