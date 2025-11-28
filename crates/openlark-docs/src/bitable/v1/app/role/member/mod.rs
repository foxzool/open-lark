use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AppRoleMember {
    service: Arc<DocsService>,
}

impl AppRoleMember {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/create
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_roles_by_role_id_members(&self, app_token: impl AsRef<str>, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/batch_create
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_roles_by_role_id_members_batch_create(&self, app_token: impl AsRef<str>, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_create".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/list
    pub async fn get_open_apis_bitable_v1_apps_by_app_token_roles_by_role_id_members(&self, app_token: impl AsRef<str>, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/delete
    pub async fn delete_open_apis_bitable_v1_apps_by_app_token_roles_by_role_id_members_by_member_id(&self, app_token: impl AsRef<str>, role_id: impl AsRef<str>, member_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/:member_id".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":role_id", role_id.as_ref());
        path = path.replace(":member_id", member_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/batch_delete
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_roles_by_role_id_members_batch_delete(&self, app_token: impl AsRef<str>, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_delete".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
