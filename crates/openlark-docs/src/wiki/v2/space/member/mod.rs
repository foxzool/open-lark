use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct SpaceMember {
    service: Arc<DocsService>,
}

impl SpaceMember {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/docs/wiki-v2/space-member/list
    pub async fn get_open_apis_wiki_v2_spaces_by_space_id_members(&self, space_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/members".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/create
    pub async fn post_open_apis_wiki_v2_spaces_by_space_id_members(&self, space_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/members".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/delete
    pub async fn delete_open_apis_wiki_v2_spaces_by_space_id_members_by_member_id(&self, space_id: impl AsRef<str>, member_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/wiki/v2/spaces/:space_id/members/:member_id".to_string();
        path = path.replace(":space_id", space_id.as_ref());
        path = path.replace(":member_id", member_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
