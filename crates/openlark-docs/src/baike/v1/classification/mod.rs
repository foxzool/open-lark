use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Classification {
    service: Arc<DocsService>,
}

impl Classification {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/baike-v1/classification/list
    pub async fn get_open_apis_baike_v1_classifications(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/baike/v1/classifications".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
