use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct JobManager {
    service: Arc<HrService>,
}

impl JobManager {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/batch_update
    pub async fn post_open_apis_hire_v1_jobs_by_job_id_managers_batch_update(&self, job_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/jobs/:job_id/managers/batch_update".to_string();
        path = path.replace(":job_id", job_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/get-2
    pub async fn get_open_apis_hire_v1_jobs_by_job_id_managers_by_manager_id(&self, job_id: impl AsRef<str>, manager_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/jobs/:job_id/managers/:manager_id".to_string();
        path = path.replace(":job_id", job_id.as_ref());
        path = path.replace(":manager_id", manager_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
