use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct CostCenterVersion {
    service: Arc<HrService>,
}

impl CostCenterVersion {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/organization-management/cost_center/cost_center-version/create
    pub async fn post_open_apis_corehr_v2_cost_centers_by_cost_center_id_versions(&self, cost_center_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/cost_centers/:cost_center_id/versions".to_string();
        path = path.replace(":cost_center_id", cost_center_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/organization-management/cost_center/cost_center-version/patch
    pub async fn patch_open_apis_corehr_v2_cost_centers_by_cost_center_id_versions_by_version_id(&self, cost_center_id: impl AsRef<str>, version_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/cost_centers/:cost_center_id/versions/:version_id".to_string();
        path = path.replace(":cost_center_id", cost_center_id.as_ref());
        path = path.replace(":version_id", version_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/organization-management/cost_center/cost_center-version/delete
    pub async fn delete_open_apis_corehr_v2_cost_centers_by_cost_center_id_versions_by_version_id(&self, cost_center_id: impl AsRef<str>, version_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/cost_centers/:cost_center_id/versions/:version_id".to_string();
        path = path.replace(":cost_center_id", cost_center_id.as_ref());
        path = path.replace(":version_id", version_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
