#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 云文档 v1 API 模块
///
/// 提供云文档内容获取相关的API功能。
use openlark_core::config::Config;

// 重新导出所有子模块
pub use content::*;

// 子模块
pub mod content;

/// 云文档 v1 服务
///
/// 提供云文档的完整管理功能，包括文档内容获取等。
/// 支持多种文档格式的内容访问。
#[derive(Clone)]
pub struct DocsService {
    config: Config,
}

impl DocsService {
    /// 创建新的云文档服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取文档内容服务
    ///
    /// 访问文档内容相关的API，包括获取文档详细内容等。
    ///
    /// # 返回
    /// 返回文档内容服务实例
    pub fn content(&self) -> crate::ccm::docs::v1::content::ContentService {
        crate::ccm::docs::v1::content::ContentService::new(self.config.clone())
    }
}

impl openlark_core::trait_system::service::Service for DocsService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "docs"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::service::Service;

    fn create_test_service() -> DocsService {
        let config = openlark_core::config::Config::builder().app_id("test_app_id").app_secret("test_app_secret").build();
        DocsService::new(config)
    }

    #[test]
    fn test_docs_service_creation() {
        let config = openlark_core::config::Config::builder().app_id("test_app_id").app_secret("test_app_secret").build();
        let service = DocsService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_docs_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(
            service.config().app_secret(),
            cloned_service.config().app_secret()
        );
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_content_service_access() {
        let service = create_test_service();
        let content_service = service.content();

        // 验证内容服务创建成功，配置信息一致
        assert_eq!(service.config().app_id(), content_service.config().app_id());
        assert_eq!(
            service.config().app_secret(),
            content_service.config().app_secret()
        );
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务
        let _content_service = service.content();

        // 如果编译通过，说明模块结构正确
        assert!(true);
    }
}
