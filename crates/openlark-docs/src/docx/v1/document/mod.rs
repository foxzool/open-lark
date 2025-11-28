pub mod block;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Document {
    service: Arc<DocsService>,
}

impl Document {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create
    pub async fn post_open_apis_docx_v1_documents(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/docx/v1/documents".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/get
    pub async fn get_open_apis_docx_v1_documents_by_document_id(&self, document_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/documents/:document_id".to_string();
        path = path.replace(":document_id", document_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/raw_content
    pub async fn get_open_apis_docx_v1_documents_by_document_id_raw_content(&self, document_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/documents/:document_id/raw_content".to_string();
        path = path.replace(":document_id", document_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert
    pub async fn post_open_apis_docx_documents_blocks_convert(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/docx/documents/blocks/convert".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
