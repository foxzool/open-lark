use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct EcoBackgroundCheckCustomField {
    service: Arc<HrService>,
}

impl EcoBackgroundCheckCustomField {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/eco_background_check_custom_field/create
    pub async fn post_open_apis_hire_v1_eco_background_check_custom_fields(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/eco_background_check_custom_fields".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/eco_background_check_custom_field/batch_update
    pub async fn patch_open_apis_hire_v1_eco_background_check_custom_fields_batch_update(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_update".to_string();
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/eco_background_check_custom_field/batch_delete
    pub async fn post_open_apis_hire_v1_eco_background_check_custom_fields_batch_delete(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_delete".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
