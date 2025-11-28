use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct SpaceNode {
    service: Arc<DocsService>,
}

impl SpaceNode {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/create
    pub async fn post_open_apis_wiki_v2_spaces_by_space_id_nodes(&self, space_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/nodes".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/list
    pub async fn get_open_apis_wiki_v2_spaces_by_space_id_nodes(&self, space_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/nodes".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/move
    pub async fn post_open_apis_wiki_v2_spaces_by_space_id_nodes_by_node_token_move(&self, space_id: impl AsRef<str>, node_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/move".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        path = path.replace(":node_token", node_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/update_title
    pub async fn post_open_apis_wiki_v2_spaces_by_space_id_nodes_by_node_token_update_title(&self, space_id: impl AsRef<str>, node_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/update_title".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        path = path.replace(":node_token", node_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/copy
    pub async fn post_open_apis_wiki_v2_spaces_by_space_id_nodes_by_node_token_copy(&self, space_id: impl AsRef<str>, node_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/copy".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        path = path.replace(":node_token", node_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/move_docs_to_wiki
    pub async fn post_open_apis_wiki_v2_spaces_by_space_id_nodes_move_docs_to_wiki(&self, space_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/nodes/move_docs_to_wiki".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
