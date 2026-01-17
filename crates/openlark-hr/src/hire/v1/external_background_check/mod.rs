use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExternalBackgroundCheck {
    service: Arc<HrService>,
}

impl ExternalBackgroundCheck {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/import-external-system-information/create-2
    pub async fn post_open_apis_hire_v1_external_background_checks(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/hire/v1/external_background_checks".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-background-info/update
    pub async fn put_open_apis_hire_v1_external_background_checks_by_external_background_check_id(
        &self,
        external_background_check_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/external_background_checks/:external_background_check_id"
                .to_string();
        path = path.replace(
            ":external_background_check_id",
            external_background_check_id.as_ref(),
        );
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-background-info/batch_query
    pub async fn post_open_apis_hire_v1_external_background_checks_batch_query(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/hire/v1/external_background_checks/batch_query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-background-info/delete
    pub async fn delete_open_apis_hire_v1_external_background_checks_by_external_background_check_id(
        &self,
        external_background_check_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/external_background_checks/:external_background_check_id"
                .to_string();
        path = path.replace(
            ":external_background_check_id",
            external_background_check_id.as_ref(),
        );
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
