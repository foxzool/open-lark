use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ImportTask {
    service: Arc<DocsService>,
}

impl ImportTask {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/create
    pub async fn post_open_apis_drive_v1_import_tasks(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let path = "/open-apis/drive/v1/import_tasks".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/get
    pub async fn get_open_apis_drive_v1_import_tasks_by_ticket(&self, ticket: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/drive/v1/import_tasks/:ticket".to_string();
        path = path.replace(":ticket", ticket.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
