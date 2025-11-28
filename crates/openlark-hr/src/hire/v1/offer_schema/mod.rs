use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct OfferSchema {
    service: Arc<HrService>,
}

impl OfferSchema {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/hire_internal/offer/get
    pub async fn get_open_apis_hire_v1_offer_schemas_by_offer_schema_id(&self, offer_schema_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/offer_schemas/:offer_schema_id".to_string();
        path = path.replace(":offer_schema_id", offer_schema_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
