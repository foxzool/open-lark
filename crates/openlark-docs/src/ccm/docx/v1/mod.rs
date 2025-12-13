#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 文档处理 v1 API 模块
///
/// 提供文档处理和聊天公告相关的API功能，包括文档创建、内容管理、区块操作等。
use openlark_core::config::Config;

// 重新导出所有子模块
pub use chat::*;
pub use document::*;

// 子模块
pub mod chat;
pub mod document;

/// 文档处理 v1 服务
///
/// 提供文档的完整管理功能，包括文档创建、内容获取、区块管理等。
/// 支持文档处理和聊天公告功能。
#[derive(Clone)]
pub struct DocxService {
    config: Config,
}

impl DocxService {
    /// 创建新的文档处理服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取文档服务
    ///
    /// 访问文档相关的API，包括文档创建、获取、内容管理等。
    ///
    /// # 返回
    /// 返回文档服务实例
    pub fn document(&self) -> crate::ccm::docx::v1::document::DocumentService {
        crate::ccm::docx::v1::document::DocumentService::new(self.config.clone())
    }

    /// 获取聊天公告服务
    ///
    /// 访问聊天公告相关的API，包括公告创建、获取、区块管理等。
    ///
    /// # 返回
    /// 返回聊天公告服务实例
    pub fn chat(&self) -> crate::ccm::docx::v1::chat::ChatService {
        crate::ccm::docx::v1::chat::ChatService::new(self.config.clone())
    }
}

impl openlark_core::trait_system::service::Service for DocxService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "docx"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> DocxService {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        DocxService::new(config)
    }

    #[test]
    fn test_docx_service_creation() {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        let service = DocxService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_docx_service_clone() {
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
    fn test_document_service_access() {
        let service = create_test_service();
        let document_service = service.document();

        // 验证文档服务创建成功，配置信息一致
        assert_eq!(
            service.config().app_id(),
            document_service.config().app_id()
        );
        assert_eq!(
            service.config().app_secret(),
            document_service.config().app_secret()
        );
    }

    #[test]
    fn test_chat_service_access() {
        let service = create_test_service();
        let chat_service = service.chat();

        // 验证聊天服务创建成功，配置信息一致
        assert_eq!(service.config().app_id(), chat_service.config().app_id());
        assert_eq!(
            service.config().app_secret(),
            chat_service.config().app_secret()
        );
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务
        let _document_service = service.document();
        let _chat_service = service.chat();

        // 如果编译通过，说明模块结构正确
        assert!(true);
    }
}
