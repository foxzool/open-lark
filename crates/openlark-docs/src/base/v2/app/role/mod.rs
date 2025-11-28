use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AppRole {
    service: Arc<DocsService>,
}

impl AppRole {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/create-2
    pub async fn post_open_apis_base_v2_apps_by_app_token_roles(&self, app_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/base/v2/apps/:app_token/roles".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/update-2
    pub async fn put_open_apis_base_v2_apps_by_app_token_roles_by_role_id(&self, app_token: impl AsRef<str>, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/base/v2/apps/:app_token/roles/:role_id".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/list-2
    pub async fn get_open_apis_base_v2_apps_by_app_token_roles(&self, app_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/base/v2/apps/:app_token/roles".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
