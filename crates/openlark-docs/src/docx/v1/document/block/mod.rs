pub mod children;
pub mod descendant;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct DocumentBlock {
    service: Arc<DocsService>,
}

impl DocumentBlock {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/list
    pub async fn get_open_apis_docx_v1_documents_by_document_id_blocks(&self, document_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/documents/:document_id/blocks".to_string();
        path = path.replace(":document_id", document_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/patch
    pub async fn patch_open_apis_docx_v1_documents_by_document_id_blocks_by_block_id(&self, document_id: impl AsRef<str>, block_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/documents/:document_id/blocks/:block_id".to_string();
        path = path.replace(":document_id", document_id.as_ref());
        path = path.replace(":block_id", block_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/get
    pub async fn get_open_apis_docx_v1_documents_by_document_id_blocks_by_block_id(&self, document_id: impl AsRef<str>, block_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/documents/:document_id/blocks/:block_id".to_string();
        path = path.replace(":document_id", document_id.as_ref());
        path = path.replace(":block_id", block_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/batch_update
    pub async fn patch_open_apis_docx_v1_documents_by_document_id_blocks_batch_update(&self, document_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/documents/:document_id/blocks/batch_update".to_string();
        path = path.replace(":document_id", document_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
