use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct JobLevel {
    service: Arc<HrService>,
}

impl JobLevel {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/job-management/job_level/create
    pub async fn post_open_apis_corehr_v1_job_levels(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/job_levels".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/job-management/job_level/patch
    pub async fn patch_open_apis_corehr_v1_job_levels_by_job_level_id(&self, job_level_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/job_levels/:job_level_id".to_string();
        path = path.replace(":job_level_id", job_level_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/job-management/job_level/get
    pub async fn get_open_apis_corehr_v1_job_levels_by_job_level_id(&self, job_level_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/job_levels/:job_level_id".to_string();
        path = path.replace(":job_level_id", job_level_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/job-management/job_level/list
    pub async fn get_open_apis_corehr_v1_job_levels(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/job_levels".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/job-management/job_level/delete
    pub async fn delete_open_apis_corehr_v1_job_levels_by_job_level_id(&self, job_level_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/job_levels/:job_level_id".to_string();
        path = path.replace(":job_level_id", job_level_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
