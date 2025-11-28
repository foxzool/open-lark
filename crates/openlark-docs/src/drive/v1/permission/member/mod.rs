use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct PermissionMember {
    service: Arc<DocsService>,
}

impl PermissionMember {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/create
    pub async fn post_open_apis_drive_v1_permissions_by_token_members(&self, token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/members".to_string();
        path = path.replace(":token", token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/docs/permission/permission-member/batch_create
    pub async fn post_open_apis_drive_v1_permissions_by_token_members_batch_create(&self, token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/members/batch_create".to_string();
        path = path.replace(":token", token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/update
    pub async fn put_open_apis_drive_v1_permissions_by_token_members_by_member_id(&self, token: impl AsRef<str>, member_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/members/:member_id".to_string();
        path = path.replace(":token", token.as_ref());
        path = path.replace(":member_id", member_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/list
    pub async fn get_open_apis_drive_v1_permissions_by_token_members(&self, token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/members".to_string();
        path = path.replace(":token", token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/delete
    pub async fn delete_open_apis_drive_v1_permissions_by_token_members_by_member_id(&self, token: impl AsRef<str>, member_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/members/:member_id".to_string();
        path = path.replace(":token", token.as_ref());
        path = path.replace(":member_id", member_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/transfer_owner
    pub async fn post_open_apis_drive_v1_permissions_by_token_members_transfer_owner(&self, token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/members/transfer_owner".to_string();
        path = path.replace(":token", token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/auth
    pub async fn get_open_apis_drive_v1_permissions_by_token_members_auth(&self, token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/permissions/:token/members/auth".to_string();
        path = path.replace(":token", token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
