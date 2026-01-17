use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct SpaceSetting {
    service: Arc<DocsService>,
}

impl SpaceSetting {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-setting/update
    pub async fn put_open_apis_wiki_v2_spaces_by_space_id_setting(&self, space_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/setting".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
