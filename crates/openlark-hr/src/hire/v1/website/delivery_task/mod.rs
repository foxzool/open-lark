use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct WebsiteDeliveryTask {
    service: Arc<HrService>,
}

impl WebsiteDeliveryTask {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/get-2
    pub async fn get_open_apis_hire_v1_websites_by_website_id_delivery_tasks_by_delivery_task_id(&self, website_id: impl AsRef<str>, delivery_task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/websites/:website_id/delivery_tasks/:delivery_task_id".to_string();
        path = path.replace(":website_id", website_id.as_ref());
        path = path.replace(":delivery_task_id", delivery_task_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
