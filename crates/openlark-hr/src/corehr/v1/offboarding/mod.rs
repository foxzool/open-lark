use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Offboarding {
    service: Arc<HrService>,
}

impl Offboarding {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/offboarding/query
    pub async fn post_open_apis_corehr_v1_offboardings_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/offboardings/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/offboarding/search
    pub async fn post_open_apis_corehr_v1_offboardings_search(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/offboardings/search".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/offboarding/submit
    pub async fn post_open_apis_corehr_v1_offboardings_submit(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/offboardings/submit".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
