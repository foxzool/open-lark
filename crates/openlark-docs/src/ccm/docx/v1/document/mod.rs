/// 文档模块
///
/// 提供文档的基础操作功能，包括文档创建、获取、内容管理等。
use openlark_core::{api::Response, config::Config, error::validation_error, SDKResult};

// 重新导出所有模块类型，解决名称冲突
// block 模块显式导出
pub use block::{
    BatchDeleteDocumentBlockChildrenParams,
    BatchDeleteDocumentBlockChildrenRequest,
    BatchDeleteDocumentBlockChildrenResponse,
    BatchUpdateDocumentBlocksParams,
    BatchUpdateDocumentBlocksRequest,
    BatchUpdateDocumentBlocksResponse,
    BatchUpdateRequest,
    BlockIdRelation,
    ContentType,
    ConvertContentToBlocksParams,
    ConvertContentToBlocksRequest,
    ConvertContentToBlocksResponse,
    CreateDocumentBlockChildrenParams,
    CreateDocumentBlockChildrenRequest,
    CreateDocumentBlockChildrenResponse,
    CreateDocumentBlockDescendantParams,
    CreateDocumentBlockDescendantRequest,
    CreateDocumentBlockDescendantResponse,
    CreateDocumentParams,
    CreateDocumentRequest,
    CreateDocumentResponse,
    CreatedDocument,
    Document,
    DocumentCover,
    DocumentDisplaySetting,
    GetDocumentBlockChildrenParams,
    GetDocumentBlockChildrenRequest,
    GetDocumentBlockChildrenResponse,
    GetDocumentBlockParams,
    GetDocumentBlockRequest,
    GetDocumentBlockResponse,
    GetDocumentBlocksParams,
    GetDocumentBlocksRequest,
    GetDocumentBlocksResponse,
    GetDocumentRawContentParams,
    GetDocumentRawContentRequest,
    GetDocumentRawContentResponse,
    GetDocumentRequest,
    GetDocumentResponse,
    UpdateDocumentBlockParams,
    UpdateDocumentBlockRequest,
    UpdateDocumentBlockResponse,
    document_id,
    execute,
    execute_with_options,
    folder_token,
    new,
    title,
};
pub use convert::{
    ContentType, ConvertContentToBlocksParams, ConvertContentToBlocksRequest,
    ConvertContentToBlocksResponse,
};
#[allow(deprecated)]
pub use create::CreateDocumentParams;
pub use create::{CreateDocumentRequest, CreateDocumentResponse, CreatedDocument};
pub use get::{Document, GetDocumentRequest, GetDocumentResponse};
pub use raw_content::{
    GetDocumentRawContentParams, GetDocumentRawContentRequest, GetDocumentRawContentResponse,
};

// 子模块
pub mod block;
mod convert;
mod create;
mod get;
mod raw_content;

/// 文档服务
///
/// 提供文档的完整管理功能，包括文档创建、内容获取、区块管理等。
/// 支持多种文档格式的内容访问和处理。
#[derive(Clone)]
pub struct DocumentService {
    config: Config,
}

impl DocumentService {
    /// 创建新的文档服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建文档（流式 Builder）
    ///
    /// 返回一个请求构建器，支持链式调用设置参数。
    ///
    /// # 示例
    /// ```rust,no_run
    /// use openlark_core::config::Config;
    /// use openlark_docs::ccm::docx::v1::document::DocumentService;
    ///
    /// let config = Config::builder()
    ///     .app_id("app_id")
    ///     .app_secret("app_secret")
    ///     .build();
    /// let service = DocumentService::new(config);
    ///
    /// // 流式 Builder 调用
    /// let _future = service
    ///     .create_builder()
    ///     .title("新文档标题")
    ///     .folder_token("folder_token")
    ///     .execute();
    /// ```
    pub fn create_builder(&self) -> CreateDocumentRequest {
        CreateDocumentRequest::new(self.config.clone())
    }

    /// 获取文档信息请求构建器
    ///
    /// 返回一个请求构建器，支持链式调用设置参数。
    pub fn get_builder(&self) -> GetDocumentRequest {
        GetDocumentRequest::new(self.config.clone())
    }
}

impl openlark_core::trait_system::service::Service for DocumentService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "document"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::service::Service;

    fn create_test_service() -> DocumentService {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        DocumentService::new(config)
    }

    #[test]
    fn test_document_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocumentService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_document_service_clone() {
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
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_create_document_builder() {
        let service = create_test_service();
        let _request = service
            .create_builder()
            .title("文档标题")
            .folder_token("folder_token");
    }

    #[test]
    fn test_get_document_builder() {
        let service = create_test_service();
        let _request = service.get_builder().document_id("document_token");
    }

    #[test]
    fn test_module_structure() {
        let service = create_test_service();
        let _create_request = service.create_builder();
        let _get_request = service.get_builder();
    }
}
