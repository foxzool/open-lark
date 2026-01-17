use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Content {
    service: Arc<DocsService>,
}

impl Content {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/docs/docs-v1/get
    pub async fn get_open_apis_docs_v1_content(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docs/v1/content".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
