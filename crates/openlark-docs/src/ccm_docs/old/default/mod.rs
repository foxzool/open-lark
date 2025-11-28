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

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
    pub async fn post_open_apis_suite_docs_api_search_object(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/suite/docs-api/search/object".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata
    pub async fn post_open_apis_suite_docs_api_meta(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/suite/docs-api/meta".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
