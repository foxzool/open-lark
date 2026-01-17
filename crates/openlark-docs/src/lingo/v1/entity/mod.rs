use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Entity {
    service: Arc<DocsService>,
}

impl Entity {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/entity/create
    pub async fn post_open_apis_lingo_v1_entities(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/entities".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/entity/update
    pub async fn put_open_apis_lingo_v1_entities_by_entity_id(&self, entity_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/entities/:entity_id".to_string();
        path = path.replace(":entity_id", entity_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/entity/delete
    pub async fn delete_open_apis_lingo_v1_entities_by_entity_id(&self, entity_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/entities/:entity_id".to_string();
        path = path.replace(":entity_id", entity_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/entity/get
    pub async fn get_open_apis_lingo_v1_entities_by_entity_id(&self, entity_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/entities/:entity_id".to_string();
        path = path.replace(":entity_id", entity_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/entity/list
    pub async fn get_open_apis_lingo_v1_entities(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/entities".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/entity/match
    pub async fn post_open_apis_lingo_v1_entities_match(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/entities/match".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/entity/search
    pub async fn post_open_apis_lingo_v1_entities_search(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/entities/search".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/lingo-v1/entity/highlight
    pub async fn post_open_apis_lingo_v1_entities_highlight(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/lingo/v1/entities/highlight".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
