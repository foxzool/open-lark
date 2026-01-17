pub mod version;

use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct CostCenter {
    service: Arc<HrService>,
}

impl CostCenter {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/organization-management/cost_center/create
    pub async fn post_open_apis_corehr_v2_cost_centers(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/cost_centers".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/organization-management/cost_center/patch
    pub async fn patch_open_apis_corehr_v2_cost_centers_by_cost_center_id(
        &self,
        cost_center_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/cost_centers/:cost_center_id".to_string();
        path = path.replace(":cost_center_id", cost_center_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/organization-management/cost_center/query_recent_change
    pub async fn get_open_apis_corehr_v2_cost_centers_query_recent_change(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/cost_centers/query_recent_change".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/organization-management/cost_center/search
    pub async fn post_open_apis_corehr_v2_cost_centers_search(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/cost_centers/search".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/organization-management/cost_center/delete
    pub async fn delete_open_apis_corehr_v2_cost_centers_by_cost_center_id(
        &self,
        cost_center_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/cost_centers/:cost_center_id".to_string();
        path = path.replace(":cost_center_id", cost_center_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
