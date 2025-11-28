use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct LocationAddress {
    service: Arc<HrService>,
}

impl LocationAddress {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/organization-management/location/location-address/delete
    pub async fn delete_open_apis_corehr_v2_locations_by_location_id_addresses_by_address_id(&self, location_id: impl AsRef<str>, address_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/locations/:location_id/addresses/:address_id".to_string();
        path = path.replace(":location_id", location_id.as_ref());
        path = path.replace(":address_id", address_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/organization-management/location/location-address/patch
    pub async fn patch_open_apis_corehr_v2_locations_by_location_id_addresses_by_address_id(&self, location_id: impl AsRef<str>, address_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/locations/:location_id/addresses/:address_id".to_string();
        path = path.replace(":location_id", location_id.as_ref());
        path = path.replace(":address_id", address_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/organization-management/location/location-address/create
    pub async fn post_open_apis_corehr_v2_locations_by_location_id_addresses(&self, location_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/locations/:location_id/addresses".to_string();
        path = path.replace(":location_id", location_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
