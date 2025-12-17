pub mod meta;
pub mod search;

pub use meta::*;
pub use search::*;

use openlark_core::config::Config;

/// Docs-API 服务（旧版）
///
/// docPath:
/// - `https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search`
/// - `https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata`
#[derive(Clone)]
pub struct DocsApiService {
    config: Config,
}

impl DocsApiService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 搜索云文档
    pub fn search_object(
        &self,
    ) -> crate::ccm::ccm_docs::old::default::docs_api::search::object::SearchObjectRequest {
        crate::ccm::ccm_docs::old::default::docs_api::search::object::SearchObjectRequest::new(
            self.config.clone(),
        )
    }

    /// 获取元数据
    pub fn meta(&self) -> crate::ccm::ccm_docs::old::default::docs_api::meta::GetMetaRequest {
        crate::ccm::ccm_docs::old::default::docs_api::meta::GetMetaRequest::new(self.config.clone())
    }
}
