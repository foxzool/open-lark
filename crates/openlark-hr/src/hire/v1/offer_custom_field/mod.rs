use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct OfferCustomField {
    service: Arc<HrService>,
}

impl OfferCustomField {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/offer-settings/offer_application_form/update
    pub async fn put_open_apis_hire_v1_offer_custom_fields_by_offer_custom_field_id(
        &self,
        offer_custom_field_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/offer_custom_fields/:offer_custom_field_id".to_string();
        path = path.replace(":offer_custom_field_id", offer_custom_field_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
