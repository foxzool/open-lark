#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// CCM 文档 v1 API 模块
///
/// 提供云文档内容管理相关的API功能，包括文档搜索和元数据获取等。
use openlark_core::config::Config;

// 重新导出所有子模块
pub use docs::*;

// 子模块
pub mod docs;

/// CCM 文档服务
///
/// 提供云文档的完整管理功能，包括文档搜索、元数据获取等。
/// 支持多种文档类型的统一管理。
#[derive(Clone)]
pub struct CcmDocsService {
    config: Config,
}

impl CcmDocsService {
    /// 创建新的CCM文档服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取文档API服务
    ///
    /// 访问文档相关的API，包括文档搜索、元数据获取等。
    ///
    /// # 返回
    /// 返回文档API服务实例
    pub fn docs(&self) -> crate::ccm::ccm_docs::docs::DocsService {
        crate::ccm::ccm_docs::docs::DocsService::new(self.config.clone())
    }
}

impl openlark_core::trait_system::service::Service for CcmDocsService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ccmdocs"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> CcmDocsService {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        CcmDocsService::new(config)
    }

    #[test]
    fn test_ccm_docs_service_creation() {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        let service = CcmDocsService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_ccm_docs_service_clone() {
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
    fn test_docs_service_access() {
        let service = create_test_service();
        let docs_service = service.docs();

        // 验证文档服务创建成功，配置信息一致
        assert_eq!(service.config().app_id(), docs_service.config().app_id());
        assert_eq!(
            service.config().app_secret(),
            docs_service.config().app_secret()
        );
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务
        let _docs_service = service.docs();

        // 如果编译通过，说明模块结构正确
        assert!(true);
    }
}
