use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct EmployeesAdditionalJob {
    service: Arc<HrService>,
}

impl EmployeesAdditionalJob {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/employee/job_data/employees-additional_job/create
    pub async fn post_open_apis_corehr_v2_employees_additional_jobs(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/employees/additional_jobs".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/employee/job_data/employees-additional_job/patch
    pub async fn patch_open_apis_corehr_v2_employees_additional_jobs_by_additional_job_id(&self, additional_job_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/employees/additional_jobs/:additional_job_id".to_string();
        path = path.replace(":additional_job_id", additional_job_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/employee/job_data/employees-additional_job/delete
    pub async fn delete_open_apis_corehr_v2_employees_additional_jobs_by_additional_job_id(&self, additional_job_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/employees/additional_jobs/:additional_job_id".to_string();
        path = path.replace(":additional_job_id", additional_job_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/employee/job_data/employees-additional_job/batch
    pub async fn post_open_apis_corehr_v2_employees_additional_jobs_batch(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/employees/additional_jobs/batch".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
