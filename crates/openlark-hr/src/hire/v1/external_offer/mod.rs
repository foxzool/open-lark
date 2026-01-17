use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExternalOffer {
    service: Arc<HrService>,
}

impl ExternalOffer {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-offer-info/create
    pub async fn post_open_apis_hire_v1_external_offers(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/hire/v1/external_offers".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-offer-info/update
    pub async fn put_open_apis_hire_v1_external_offers_by_external_offer_id(
        &self,
        external_offer_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/external_offers/:external_offer_id".to_string();
        path = path.replace(":external_offer_id", external_offer_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-offer-info/batch_query
    pub async fn post_open_apis_hire_v1_external_offers_batch_query(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/hire/v1/external_offers/batch_query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-offer-info/delete
    pub async fn delete_open_apis_hire_v1_external_offers_by_external_offer_id(
        &self,
        external_offer_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/external_offers/:external_offer_id".to_string();
        path = path.replace(":external_offer_id", external_offer_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
