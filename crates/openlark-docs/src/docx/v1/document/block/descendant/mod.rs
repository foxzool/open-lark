use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct DocumentBlockDescendant {
    service: Arc<DocsService>,
}

impl DocumentBlockDescendant {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/docs/docs/document-block/create-2
    pub async fn post_open_apis_docx_v1_documents_by_document_id_blocks_by_block_id_descendant(&self, document_id: impl AsRef<str>, block_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/documents/:document_id/blocks/:block_id/descendant".to_string();
        path = path.replace(":document_id", document_id.as_ref());
        path = path.replace(":block_id", block_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
