#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// CCM Doc V1 API 模块
///
/// 旧版文档相关API实现，包含6个API：
/// - create: 创建旧版文档
/// - meta: 获取旧版文档元信息
/// - sheet_meta: 获取旧版文档中的电子表格元数据
/// - raw_content: 获取旧版文档纯文本内容
/// - content: 获取旧版文档富文本内容
/// - batch_update: 编辑旧版文档内容
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};
use serde_json::json;

// 导入子模块
pub mod batch_update;
pub mod content;
pub mod create;
pub mod meta;
pub mod models;
pub mod raw_content;
pub mod requests;
pub mod responses;
pub mod sheet_meta;

// 重新导出所有子模块类型
pub use batch_update::*;
pub use content::*;
pub use create::*;
pub use meta::*;
pub use models::*;
pub use raw_content::*;
pub use requests::*;
pub use responses::*;
pub use sheet_meta::*;

/// CCM Doc V1 API服务
///
/// 提供旧版文档的完整管理功能，包括文档创建、元信息获取、内容操作等。
/// 支持多种文档类型的统一管理。
#[derive(Clone)]
pub struct CcmDocV1Service {
    config: Config,
}

impl CcmDocV1Service {
    /// 创建新的CCM Doc V1服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建旧版文档
    ///
    /// 创建一个新的旧版文档，支持指定文档类型、标题等。
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
    /// use open_lark::service::cloud_docs::ccm_doc::v1::{CcmDocV1Service, CreateDocumentRequest};
    ///
    /// let service = CcmDocV1Service::new(config);
    /// let request = CreateDocumentRequest::new("文档标题").parent_type("doc");
    ///
    /// let response = service.create(request, None).await?;
    /// println!("文档token: {}", response.document_token);
    /// ```
    pub async fn create(
        &self,
        request: CreateDocumentRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<CreatedDocument> {
        create_document(request, &self.config, option).await
    }

    /// 获取旧版文档元信息
    ///
    /// 获取指定文档的元数据信息，包括标题、创建时间、权限等。
    ///
    /// # 参数
    /// * `request` - 获取文档元信息请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回文档元信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_doc::v1::{CcmDocV1Service, DocumentMetaRequest};
    ///
    /// let service = CcmDocV1Service::new(config);
    /// let request = DocumentMetaRequest::new("document_token");
    ///
    /// let response = service.meta(request, None).await?;
    /// println!("文档标题: {}", response.title);
    /// ```
    pub async fn meta(
        &self,
        request: DocumentMetaRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<DocumentMetaData> {
        meta::get_document_meta(request, &self.config, option).await
    }

    /// 获取旧版文档中的电子表格元数据
    ///
    /// 获取电子表格文档的工作表信息、单元格范围等元数据。
    ///
    /// # 参数
    /// * `request` - 获取电子表格元数据请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回电子表格元数据，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_doc::v1::{CcmDocV1Service, SheetMetaRequest};
    ///
    /// let service = CcmDocV1Service::new(config);
    /// let request = SheetMetaRequest::new("spreadsheet_token");
    ///
    /// let response = service.sheet_meta(request, None).await?;
    /// println!("工作表数量: {}", response.sheets.len());
    /// ```
    pub async fn sheet_meta(
        &self,
        request: SheetMetaRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<SheetMetaData> {
        sheet_meta::get_sheet_meta(request, &self.config, option).await
    }

    /// 获取旧版文档纯文本内容
    ///
    /// 获取文档的纯文本格式内容，适用于简单的文本处理。
    ///
    /// # 参数
    /// * `request` - 获取纯文本内容请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回纯文本内容，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_doc::v1::{CcmDocV1Service, RawContentRequest};
    ///
    /// let service = CcmDocV1Service::new(config);
    /// let request = RawContentRequest::new("document_token");
    ///
    /// let response = service.raw_content(request, None).await?;
    /// println!("文档内容: {}", response.content);
    /// ```
    pub async fn raw_content(
        &self,
        request: RawContentRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<responses::RawContentData> {
        raw_content::get_raw_content(request, &self.config, option).await
    }

    /// 获取旧版文档富文本内容
    ///
    /// 获取文档的富文本格式内容，保留格式信息。
    ///
    /// # 参数
    /// * `request` - 获取富文本内容请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回富文本内容，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_doc::v1::{CcmDocV1Service, DocumentContentRequest};
    ///
    /// let service = CcmDocV1Service::new(config);
    /// let request = DocumentContentRequest::new("document_token");
    ///
    /// let response = service.content(request, None).await?;
    /// println!("文档内容元素: {}", response.elements.len());
    /// ```
    pub async fn content(
        &self,
        request: DocumentContentRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<DocumentContentData> {
        content::get_document_content(request, &self.config, option).await
    }

    /// 编辑旧版文档内容
    ///
    /// 批量更新文档内容，支持插入、删除、修改等操作。
    ///
    /// # 参数
    /// * `request` - 编辑文档内容请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回更新结果，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::ccm_doc::v1::{CcmDocV1Service, BatchUpdateRequest};
    ///
    /// let service = CcmDocV1Service::new(config);
    /// let request = BatchUpdateRequest::new("document_token");
    ///
    /// let response = service.batch_update(request, None).await?;
    /// println!("更新成功");
    /// ```
    pub async fn batch_update(
        &self,
        request: BatchUpdateRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> SDKResult<responses::BatchUpdateData> {
        batch_update_document(request, &self.config, option).await
    }
}

impl openlark_core::trait_system::service::Service for CcmDocV1Service {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ccmdocv1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::service::Service;

    fn create_test_service() -> CcmDocV1Service {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        CcmDocV1Service::new(config)
    }

    #[test]
    fn test_ccm_doc_v1_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = CcmDocV1Service::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_ccm_doc_v1_service_clone() {
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
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务方法
        let _create_request = CreateDocumentRequest::new("文档标题").parent_type("doc");
        let _meta_request = DocumentMetaRequest::new("document_token");
        let _sheet_meta_request = SheetMetaRequest::new("spreadsheet_token");
        let _raw_content_request = RawContentRequest::new("document_token");
        let _content_request = DocumentContentRequest::new("document_token");
        let _batch_update_request = BatchUpdateRequest::new("document_token");

        // 如果编译通过，说明模块结构正确
        assert!(true);
    }
}
