use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AppWorkflow {
    service: Arc<DocsService>,
}

impl AppWorkflow {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/docs/bitable-v1/app-workflow/list
    pub async fn get_open_apis_bitable_v1_apps_by_app_token_workflows(&self, app_token: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/workflows".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/docs/bitable-v1/app-workflow/update
    pub async fn put_open_apis_bitable_v1_apps_by_app_token_workflows_by_workflow_id(&self, app_token: impl AsRef<str>, workflow_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/bitable/v1/apps/:app_token/workflows/:workflow_id".to_string();
        path = path.replace(":app_token", app_token.as_ref());
        path = path.replace(":workflow_id", workflow_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
