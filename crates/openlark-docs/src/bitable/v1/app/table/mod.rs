pub mod field;
pub mod form;
pub mod record;
pub mod view;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AppTable {
    service: Arc<DocsService>,
}

impl AppTable {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/create
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables(&self, app_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_create
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables_batch_create(&self, app_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/batch_create".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/patch
    pub async fn patch_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/list
    pub async fn get_open_apis_bitable_v1_apps_by_app_token_tables(&self, app_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/delete
    pub async fn delete_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_delete
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables_batch_delete(&self, app_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/batch_delete".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
