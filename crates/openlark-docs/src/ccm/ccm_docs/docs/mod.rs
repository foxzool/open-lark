#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

//! 文档（ccm_docs）API 模块
//!
//! 该模块对外保持 `crate::ccm::ccm_docs::docs::DocsService` 的入口，
//! 实际 API 实现位于 `crate::ccm::ccm_docs::old::default::docs_api`。
//!
//! docPath:
//! - `https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search`
//! - `https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata`

use openlark_core::config::Config;

/// 文档API服务
///
/// 提供文档的完整管理功能，包括文档搜索、元数据获取等。
/// 支持多种文档类型的统一管理。
#[derive(Clone)]
pub struct DocsService {
    config: Config,
}

impl DocsService {
    /// 创建新的文档API服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 搜索云文档
    #[allow(deprecated)]
    pub fn search_object(
        &self,
    ) -> crate::ccm::ccm_docs::old::default::docs_api::search::object::SearchObjectRequest {
        crate::ccm::ccm_docs::old::default::docs_api::search::object::SearchObjectRequest::new(
            self.config.clone(),
        )
    }

    /// 获取元数据
    #[allow(deprecated)]
    pub fn meta(&self) -> crate::ccm::ccm_docs::old::default::docs_api::meta::GetMetaRequest {
        crate::ccm::ccm_docs::old::default::docs_api::meta::GetMetaRequest::new(self.config.clone())
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
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        DocsService::new(config)
    }

    #[test]
    fn test_docs_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
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

    // #[test]
    // fn test_search_object_builder() {
    //     let request = SearchObjectRequest::new("搜索关键词")
    //         .page_size(20)
    //         .obj_type("doc");
    //
    //     assert_eq!(request.query, "搜索关键词");
    //     assert_eq!(request.page_size, Some(20));
    //     assert_eq!(request.obj_type, Some("doc".to_string()));
    // }

    // #[test]
    // fn test_get_meta_builder() {
    //     let request = GetMetaRequest::new("document_token");
    //
    //     assert_eq!(request.document_token, "document_token");
    // }

    // #[test]
    // fn test_module_structure() {
    //     // 这个测试验证模块结构的完整性
    //     let service = create_test_service();
    //
    //     // 验证可以访问所有服务方法
    //     let _search_request = SearchObjectRequest::new("搜索关键词");
    //     let _meta_request = GetMetaRequest::new("document_token");
    //
    //     // 如果编译通过，说明模块结构正确
    //     assert!(true);
    // }
}
