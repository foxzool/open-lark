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

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission
    pub async fn post_open_apis_drive_permission_member_permitted(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/permission/member/permitted".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/transfer-ownership
    pub async fn post_open_apis_drive_permission_member_transfer(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/permission/member/transfer".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2
    pub async fn post_open_apis_drive_permission_v2_public(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/permission/v2/public/".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
