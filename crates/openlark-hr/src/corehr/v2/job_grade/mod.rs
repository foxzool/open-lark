use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct JobGrade {
    service: Arc<HrService>,
}

impl JobGrade {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/job-management/job_grade/create
    pub async fn post_open_apis_corehr_v2_job_grades(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/job_grades".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/job-management/job_grade/patch
    pub async fn patch_open_apis_corehr_v2_job_grades_by_job_grade_id(&self, job_grade_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/job_grades/:job_grade_id".to_string();
        path = path.replace(":job_grade_id", job_grade_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/job-management/job_grade/query
    pub async fn post_open_apis_corehr_v2_job_grades_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/job_grades/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/query_recent_change
    pub async fn get_open_apis_corehr_v2_job_grades_query_recent_change(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/job_grades/query_recent_change".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/job-management/job_grade/delete
    pub async fn delete_open_apis_corehr_v2_job_grades_by_job_grade_id(&self, job_grade_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/job_grades/:job_grade_id".to_string();
        path = path.replace(":job_grade_id", job_grade_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
