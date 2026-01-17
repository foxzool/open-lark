use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Advertisement {
    service: Arc<HrService>,
}

impl Advertisement {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/publish
    pub async fn post_open_apis_hire_v1_advertisements_by_advertisement_id_publish(
        &self,
        advertisement_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/advertisements/:advertisement_id/publish".to_string();
        path = path.replace(":advertisement_id", advertisement_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
