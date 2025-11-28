use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ExternalInterviewAssessment {
    service: Arc<HrService>,
}

impl ExternalInterviewAssessment {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/import-external-system-information/create-4
    pub async fn post_open_apis_hire_v1_external_interview_assessments(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/external_interview_assessments".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-interview-info/patch
    pub async fn patch_open_apis_hire_v1_external_interview_assessments_by_external_interview_assessment_id(&self, external_interview_assessment_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/external_interview_assessments/:external_interview_assessment_id".to_string();
        path = path.replace(":external_interview_assessment_id", external_interview_assessment_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
