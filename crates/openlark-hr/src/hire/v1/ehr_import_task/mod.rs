use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct EhrImportTask {
    service: Arc<HrService>,
}

impl EhrImportTask {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/onboard/patch-2
    pub async fn patch_open_apis_hire_v1_ehr_import_tasks_by_ehr_import_task_id(&self, ehr_import_task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/ehr_import_tasks/:ehr_import_task_id".to_string();
        path = path.replace(":ehr_import_task_id", ehr_import_task_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
