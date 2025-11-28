use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct InterviewRegistrationSchema {
    service: Arc<HrService>,
}

impl InterviewRegistrationSchema {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/interview-settings/list-3
    pub async fn get_open_apis_hire_v1_interview_registration_schemas(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/interview_registration_schemas".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
