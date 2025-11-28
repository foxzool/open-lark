pub mod approver;
pub mod extra;
pub mod form_variable_data;
pub mod transfer;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Process {
    service: Arc<HrService>,
}

impl Process {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/process-form_variable_data/process-instance/list
    pub async fn get_open_apis_corehr_v2_processes(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/processes".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/process-form_variable_data/process-instance/get
    pub async fn get_open_apis_corehr_v2_processes_by_process_id(&self, process_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/processes/:process_id".to_string();
        path = path.replace(":process_id", process_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/process-form_variable_data/process-instance/flow_variable_data
    pub async fn get_open_apis_corehr_v2_processes_by_process_id_flow_variable_data(&self, process_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/processes/:process_id/flow_variable_data".to_string();
        path = path.replace(":process_id", process_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
