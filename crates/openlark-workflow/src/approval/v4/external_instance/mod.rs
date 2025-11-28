use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ExternalInstance {
    service: Arc<WorkflowService>,
}

impl ExternalInstance {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/external_instance/create
    pub async fn post_open_apis_approval_v4_external_instances(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/external_instances".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/external_instance/check
    pub async fn post_open_apis_approval_v4_external_instances_check(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/external_instances/check".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
