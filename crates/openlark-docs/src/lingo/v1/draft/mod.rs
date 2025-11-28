use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Draft {
    service: Arc<DocsService>,
}

impl Draft {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/draft/create
    pub async fn post_open_apis_lingo_v1_drafts(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/drafts".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/draft/update
    pub async fn put_open_apis_lingo_v1_drafts_by_draft_id(&self, draft_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/drafts/:draft_id".to_string();
        path = path.replace(":draft_id", draft_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
