use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Review {
    service: Arc<HrService>,
}

impl Review {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/okr-v1/review/query
    pub async fn get_open_apis_okr_v1_reviews_query(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/okr/v1/reviews/query".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
