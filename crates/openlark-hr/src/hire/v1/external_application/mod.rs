use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExternalApplication {
    service: Arc<HrService>,
}

impl ExternalApplication {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/import-external-system-information/create
    pub async fn post_open_apis_hire_v1_external_applications(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/external_applications".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/import-external-system-information/update-2
    pub async fn put_open_apis_hire_v1_external_applications_by_external_application_id(
        &self,
        external_application_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/external_applications/:external_application_id".to_string();
        path = path.replace(":external_application_id", external_application_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/import-external-system-information/list
    pub async fn get_open_apis_hire_v1_external_applications(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/external_applications".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/import-external-system-information/delete
    pub async fn delete_open_apis_hire_v1_external_applications_by_external_application_id(
        &self,
        external_application_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/external_applications/:external_application_id".to_string();
        path = path.replace(":external_application_id", external_application_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
