use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ExternalApproval {
    service: Arc<WorkflowService>,
}

impl ExternalApproval {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/external_approval/create
    pub async fn post_open_apis_approval_v4_external_approvals(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/external_approvals".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/approval-v4/external_approval/get
    pub async fn get_open_apis_approval_v4_external_approvals_by_approval_code(&self, approval_code: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/external_approvals/:approval_code".to_string();
        path = path.replace(":approval_code", approval_code.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
