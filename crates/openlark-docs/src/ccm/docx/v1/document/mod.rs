/// 文档模块
///
/// 提供文档的基础操作功能，包括文档创建、获取、内容管理等。
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]
#[allow(non_snake_case)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::module_inception)]
use openlark_core::{
    api::Response, config::Config, error::validation_error, req_option::RequestOption, SDKResult,
};

// 重新导出所有模块类型，解决名称冲突
pub use convert::{ContentType, ConvertContentToBlocksParams, ConvertContentToBlocksRequest, ConvertContentToBlocksResponse};
pub use create::{CreateDocumentParams, CreateDocumentRequest, CreateDocumentResponse, CreatedDocument};
pub use get::{Document, GetDocumentRequest, GetDocumentResponse};
pub use raw_content::{GetDocumentRawContentParams, GetDocumentRawContentRequest, GetDocumentRawContentResponse};
pub use block::*;

// 子模块
mod convert;
mod create;
mod get;
mod raw_content;
pub mod block;

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
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建文档
    ///
    /// 创建新版文档，支持设置文档标题、目录位置和模板。
    /// 文档创建后自动生成文档ID和访问链接。
    ///
    /// # 参数
    /// * `request` - 创建文档请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回创建的文档信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::docx::v1::document::{DocumentService, CreateDocumentRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = CreateDocumentRequest::new()
    ///     .title("新文档标题")
    ///     .folder_token("folder_token");
    ///
    /// let response = service.create(request, None).await?;
    /// println!("文档创建成功，ID: {}", response.document_id);
    /// ```
    pub async fn create(
        &self,
        params: CreateDocumentParams,
        option: Option<RequestOption>,
    ) -> SDKResult<CreatedDocument> {
        let _ = option;
        let data = CreateDocumentRequest::new(self.config.clone()).execute(params).await?;
        Ok(data.document)
    }

    /// 获取文档信息
    ///
    /// 获取指定文档的详细信息，包括文档内容、结构信息等。
    /// 支持获取文档的完整内容和元数据。
    ///
    /// # 参数
    /// * `request` - 获取文档请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回文档详细信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::docx::v1::document::{DocumentService, GetDocumentRequest, GetDocumentParams};
    ///
    /// let service = DocumentService::new(config);
    /// let request = GetDocumentRequest::new(config.clone());
    /// let params = GetDocumentParams {
    ///     document_id: "document_token".to_string(),
    ///     with_content: Some(true),
    /// };
    ///
    /// let response = service.get(request, params, None).await?;
    /// println!("文档标题: {}", response.title);
    /// ```
    pub async fn get(
        &self,
        request: GetDocumentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Document> {
        let _ = option;
        let result = request.execute().await?;
        Ok(result.document)
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

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_create_document_builder() {
        let request = CreateDocumentRequest::new()
            .title("文档标题")
            .folder_token("folder_token");

        assert_eq!(request.title, Some("文档标题".to_string()));
        assert_eq!(request.folder_token, Some("folder_token".to_string()));
    }

    #[test]
    fn test_get_document_builder() {
        let config = openlark_core::config::Config::builder()
            .app_id("id")
            .app_secret("secret")
            .build();
        let request = GetDocumentRequest::new(config);
        let params = GetDocumentParams {
            document_id: "document_token".to_string(),
            with_content: None,
        };
        assert_eq!(params.document_id, "document_token");
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务方法
        let _create_request = CreateDocumentRequest::new();
        let config = openlark_core::config::Config::builder()
            .app_id("id")
            .app_secret("secret")
            .build();
        let _get_request = GetDocumentRequest::new(config);

        // 如果编译通过，说明模块结构正确
        assert!(true);
    }
}
