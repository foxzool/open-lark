pub mod attachment;

use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct InterviewRecord {
    service: Arc<HrService>,
}

impl InterviewRecord {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/interview/get
    pub async fn get_open_apis_hire_v1_interview_records_by_interview_record_id(
        &self,
        interview_record_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/interview_records/:interview_record_id".to_string();
        path = path.replace(":interview_record_id", interview_record_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/candidate-management/delivery-process-management/interview/list-3
    pub async fn get_open_apis_hire_v1_interview_records(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/interview_records".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
