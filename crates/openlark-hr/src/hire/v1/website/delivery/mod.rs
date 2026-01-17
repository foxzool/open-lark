use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct WebsiteDelivery {
    service: Arc<HrService>,
}

impl WebsiteDelivery {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/create_by_resume
    pub async fn post_open_apis_hire_v1_websites_by_website_id_deliveries_create_by_resume(
        &self,
        website_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/websites/:website_id/deliveries/create_by_resume".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/create_by_attachment
    pub async fn post_open_apis_hire_v1_websites_by_website_id_deliveries_create_by_attachment(
        &self,
        website_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/websites/:website_id/deliveries/create_by_attachment".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
