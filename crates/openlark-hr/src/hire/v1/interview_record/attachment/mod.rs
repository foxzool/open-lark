use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct InterviewRecordAttachment {
    service: Arc<HrService>,
}

impl InterviewRecordAttachment {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/interview/get-2
    pub async fn get_open_apis_hire_v1_interview_records_attachments(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/hire/v1/interview_records/attachments".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
