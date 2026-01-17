use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct RecurringPayment {
    service: Arc<HrService>,
}

impl RecurringPayment {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/recurring_payment/query
    pub async fn post_open_apis_compensation_v1_recurring_payment_query(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/compensation/v1/recurring_payment/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/recurring_payment/batch_update
    pub async fn post_open_apis_compensation_v1_recurring_payment_batch_update(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/compensation/v1/recurring_payment/batch_update".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/recurring_payment/batch_remove
    pub async fn post_open_apis_compensation_v1_recurring_payment_batch_remove(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/compensation/v1/recurring_payment/batch_remove".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/basic-compensation/recurring_payment/batch_create
    pub async fn post_open_apis_compensation_v1_recurring_payment_batch_create(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/compensation/v1/recurring_payment/batch_create".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
