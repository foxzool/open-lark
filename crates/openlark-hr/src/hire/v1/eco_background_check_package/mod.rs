use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct EcoBackgroundCheckPackage {
    service: Arc<HrService>,
}

impl EcoBackgroundCheckPackage {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/eco_background_check_package/create
    pub async fn post_open_apis_hire_v1_eco_background_check_packages(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/eco_background_check_packages".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/eco_background_check_package/batch_update
    pub async fn patch_open_apis_hire_v1_eco_background_check_packages_batch_update(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/eco_background_check_packages/batch_update".to_string();
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/eco_background_check_package/batch_delete
    pub async fn post_open_apis_hire_v1_eco_background_check_packages_batch_delete(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/eco_background_check_packages/batch_delete".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
