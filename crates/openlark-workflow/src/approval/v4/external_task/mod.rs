use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ExternalTask {
    service: Arc<WorkflowService>,
}

impl ExternalTask {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/external_task/list
    pub async fn get_open_apis_approval_v4_external_tasks(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/external_tasks".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
