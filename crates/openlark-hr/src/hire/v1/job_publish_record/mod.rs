use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct JobPublishRecord {
    service: Arc<HrService>,
}

impl JobPublishRecord {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/search
    pub async fn post_open_apis_hire_v1_job_publish_records_search(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/job_publish_records/search".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
