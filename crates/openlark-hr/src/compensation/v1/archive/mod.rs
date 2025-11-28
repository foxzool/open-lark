use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Archive {
    service: Arc<HrService>,
}

impl Archive {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/archive/create
    pub async fn post_open_apis_compensation_v1_archives(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/compensation/v1/archives".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/archive/query
    pub async fn post_open_apis_compensation_v1_archives_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/compensation/v1/archives/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
