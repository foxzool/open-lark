use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Indicator {
    service: Arc<HrService>,
}

impl Indicator {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/performance-v1/review_config/review_template/query-3
    pub async fn post_open_apis_performance_v2_indicators_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/performance/v2/indicators/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
