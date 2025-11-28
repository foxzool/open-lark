use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ProcessApprover {
    service: Arc<HrService>,
}

impl ProcessApprover {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/process-form_variable_data/approver-task/update
    pub async fn put_open_apis_corehr_v2_processes_by_process_id_approvers_by_approver_id(&self, process_id: impl AsRef<str>, approver_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/processes/:process_id/approvers/:approver_id".to_string();
        path = path.replace(":process_id", process_id.as_ref());
        path = path.replace(":approver_id", approver_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
