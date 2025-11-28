use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct MetricTag {
    service: Arc<HrService>,
}

impl MetricTag {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/performance-v1/review_config/metric_template/list
    pub async fn get_open_apis_performance_v2_metric_tags(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/performance/v2/metric_tags".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
