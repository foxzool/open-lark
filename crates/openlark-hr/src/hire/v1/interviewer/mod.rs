use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Interviewer {
    service: Arc<HrService>,
}

impl Interviewer {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/recruitment-related-configuration/interview-settings/interviewer/list
    pub async fn get_open_apis_hire_v1_interviewers(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/interviewers".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/recruitment-related-configuration/interview-settings/interviewer/patch
    pub async fn patch_open_apis_hire_v1_interviewers_by_interviewer_id(&self, interviewer_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/interviewers/:interviewer_id".to_string();
        path = path.replace(":interviewer_id", interviewer_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
