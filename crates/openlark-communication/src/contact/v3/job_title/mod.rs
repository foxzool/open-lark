use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct JobTitle {
    service: Arc<CommunicationService>,
}

impl JobTitle {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/contact-v3/job_title/get
    pub async fn get_open_apis_contact_v3_job_titles_by_job_title_id(&self, job_title_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/job_titles/:job_title_id".to_string();
        path = path.replace(":job_title_id", job_title_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/contact-v3/job_title/list
    pub async fn get_open_apis_contact_v3_job_titles(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/job_titles".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
