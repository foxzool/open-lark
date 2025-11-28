pub mod member;
pub mod node;
pub mod setting;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Space {
    service: Arc<DocsService>,
}

impl Space {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/list
    pub async fn get_open_apis_wiki_v2_spaces(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get
    pub async fn get_open_apis_wiki_v2_spaces_by_space_id(&self, space_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create
    pub async fn post_open_apis_wiki_v2_spaces(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/get_node
    pub async fn get_open_apis_wiki_v2_spaces_get_node(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/get_node".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
