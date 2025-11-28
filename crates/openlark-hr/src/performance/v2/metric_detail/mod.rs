use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct MetricDetail {
    service: Arc<HrService>,
}

impl MetricDetail {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/performance-v1/metric_detail/query
    pub async fn post_open_apis_performance_v2_metric_details_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/performance/v2/metric_details/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/performance-v1/metric_detail/import
    pub async fn post_open_apis_performance_v2_metric_details_import(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/performance/v2/metric_details/import".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
