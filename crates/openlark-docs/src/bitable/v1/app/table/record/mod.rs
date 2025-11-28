use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AppTableRecord {
    service: Arc<DocsService>,
}

impl AppTableRecord {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/create
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/update
    pub async fn put_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records_by_record_id(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, record_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        path = path.replace(":record_id", record_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/docs/bitable-v1/app-table-record/search
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records_search(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/search".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/delete
    pub async fn delete_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records_by_record_id(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, record_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        path = path.replace(":record_id", record_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_create
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records_batch_create(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_create".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_update
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records_batch_update(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_update".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/docs/bitable-v1/app-table-record/batch_get
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records_batch_get(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_get".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_delete
    pub async fn post_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records_batch_delete(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_delete".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/get
    pub async fn get_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records_by_record_id(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, record_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        path = path.replace(":record_id", record_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/list
    pub async fn get_open_apis_bitable_v1_apps_by_app_token_tables_by_table_id_records(&self, app_token: impl AsRef<str>, table_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":table_id", table_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
