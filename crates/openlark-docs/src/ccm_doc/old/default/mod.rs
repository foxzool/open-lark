use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Default {
    service: Arc<DocsService>,
}

impl Default {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docs/apiRef/create-document
    pub async fn post_open_apis_doc_v2_create(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/doc/v2/create".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta
    pub async fn get_open_apis_doc_v2_meta_by_doctoken(&self, docToken: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/doc/v2/meta/:docToken".to_string();
        path = path.replace(":docToken", docToken.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc
    pub async fn get_open_apis_doc_v2_by_doctoken_sheet_meta(&self, docToken: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/doc/v2/:docToken/sheet_meta".to_string();
        path = path.replace(":docToken", docToken.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content
    pub async fn get_open_apis_doc_v2_by_doctoken_raw_content(&self, docToken: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/doc/v2/:docToken/raw_content".to_string();
        path = path.replace(":docToken", docToken.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document
    pub async fn get_open_apis_doc_v2_by_doctoken_content(&self, docToken: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/doc/v2/:docToken/content".to_string();
        path = path.replace(":docToken", docToken.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document
    pub async fn post_open_apis_doc_v2_by_doctoken_batch_update(&self, docToken: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/doc/v2/:docToken/batch_update".to_string();
        path = path.replace(":docToken", docToken.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
