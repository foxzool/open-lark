use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Location {
    service: Arc<HrService>,
}

impl Location {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/recruitment-related-configuration/location/query
    pub async fn post_open_apis_hire_locations_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/locations/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/location/list
    pub async fn get_open_apis_hire_v1_locations(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/locations".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
