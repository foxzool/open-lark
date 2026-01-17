use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Repo {
    service: Arc<DocsService>,
}

impl Repo {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/repo/list
    pub async fn get_open_apis_lingo_v1_repos(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/repos".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
