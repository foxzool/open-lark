use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct PermissionPublicPassword {
    service: Arc<DocsService>,
}

impl PermissionPublicPassword {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/permission-public-password/create
    pub async fn post_open_apis_drive_v1_permissions_by_token_public_password(&self, token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/public/password".to_string();
        path = path.replace(":token", token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/permission-public-password/update
    pub async fn put_open_apis_drive_v1_permissions_by_token_public_password(&self, token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/public/password".to_string();
        path = path.replace(":token", token.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/permission-public-password/delete
    pub async fn delete_open_apis_drive_v1_permissions_by_token_public_password(&self, token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/public/password".to_string();
        path = path.replace(":token", token.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
