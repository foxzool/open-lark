use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Semester {
    service: Arc<HrService>,
}

impl Semester {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/performance-v1/review_config/semester_activity/semester/list
    pub async fn get_open_apis_performance_v1_semesters(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/performance/v1/semesters".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
