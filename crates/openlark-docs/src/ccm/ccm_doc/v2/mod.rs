/// CCM Doc v2 API - 旧版文档管理
///
/// 提供文档的创建、查询、内容管理等功能。
use openlark_core::config::Config;

pub mod doc;
pub mod models;

/// CCM Doc V2 服务
#[derive(Debug, Clone)]
pub struct CcmDocV2 {
    config: Config,
}

impl CcmDocV2 {
    /// 创建新的V2服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建旧版文档
    /// docPath: https://open.feishu.cn/document/ukTMukTMukTM/ugDM2YjL4AjN24COwYjN
    pub async fn create_document(
        &self,
        params: crate::ccm::ccm_doc::v2::models::CreateDocumentParams,
    ) -> openlark_core::SDKResult<crate::ccm::ccm_doc::v2::models::CreateDocumentResponse> {
        crate::ccm::ccm_doc::v2::doc::create_document(&self.config, params).await
    }

    /// 获取旧版文档元信息
    /// docPath: https://open.feishu.cn/document/ukTMukTMukTM/uczN3UjL3czN14yN3cTN
    pub async fn get_document_meta(
        &self,
        doc_token: &str,
    ) -> openlark_core::SDKResult<crate::ccm::ccm_doc::v2::models::DocumentMetaResponse> {
        crate::ccm::ccm_doc::v2::doc::get_document_meta(&self.config, doc_token).await
    }

    /// 获取旧版文档中的电子表格元数据
    /// docPath: https://open.feishu.cn/document/ukTMukTMukTM/uADOzUjLwgzM14CM4MTN
    pub async fn get_sheet_meta(
        &self,
        doc_token: &str,
    ) -> openlark_core::SDKResult<crate::ccm::ccm_doc::v2::models::SheetMetaResponse> {
        crate::ccm::ccm_doc::v2::doc::get_sheet_meta(&self.config, doc_token).await
    }

    /// 获取旧版文档纯文本内容
    /// docPath: https://open.feishu.cn/document/ukTMukTMukTM/ukzNzUjL5czM14SO3MTN
    pub async fn get_raw_content(
        &self,
        doc_token: &str,
    ) -> openlark_core::SDKResult<crate::ccm::ccm_doc::v2::models::RawContentResponse> {
        crate::ccm::ccm_doc::v2::doc::get_raw_content(&self.config, doc_token).await
    }

    /// 获取旧版文档富文本内容
    /// docPath: https://open.feishu.cn/document/ukTMukTMukTM/uUDM2YjL1AjN24SNwYjN
    pub async fn get_document_content(
        &self,
        doc_token: &str,
    ) -> openlark_core::SDKResult<crate::ccm::ccm_doc::v2::models::DocumentContentResponse> {
        crate::ccm::ccm_doc::v2::doc::get_document_content(&self.config, doc_token).await
    }

    /// 编辑旧版文档内容
    /// docPath: https://open.feishu.cn/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN
    pub async fn batch_update_document(
        &self,
        doc_token: &str,
        params: crate::ccm::ccm_doc::v2::models::BatchUpdateParams,
    ) -> openlark_core::SDKResult<crate::ccm::ccm_doc::v2::models::BatchUpdateResponse> {
        crate::ccm::ccm_doc::v2::doc::batch_update_document(&self.config, doc_token, params).await
    }
}

// 重新导出所有API函数
pub use doc::*;
pub use models::*;
