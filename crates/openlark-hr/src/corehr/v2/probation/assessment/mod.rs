use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProbationAssessment {
    service: Arc<HrService>,
}

impl ProbationAssessment {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/create
    pub async fn post_open_apis_corehr_v2_probation_assessments(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/corehr/v2/probation/assessments".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/patch
    pub async fn patch_open_apis_corehr_v2_probation_assessments_by_assessment_id(
        &self,
        assessment_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/probation/assessments/:assessment_id".to_string();
        path = path.replace(":assessment_id", assessment_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/delete
    pub async fn delete_open_apis_corehr_v2_probation_assessments_by_assessment_id(
        &self,
        assessment_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/probation/assessments/:assessment_id".to_string();
        path = path.replace(":assessment_id", assessment_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
