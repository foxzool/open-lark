use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct NationalIdType {
    service: Arc<HrService>,
}

impl NationalIdType {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/national_id_type/create
    pub async fn post_open_apis_corehr_v1_national_id_types(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/national_id_types".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/national_id_type/delete
    pub async fn delete_open_apis_corehr_v1_national_id_types_by_national_id_type_id(
        &self,
        national_id_type_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/national_id_types/:national_id_type_id".to_string();
        path = path.replace(":national_id_type_id", national_id_type_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/national_id_type/patch
    pub async fn patch_open_apis_corehr_v1_national_id_types_by_national_id_type_id(
        &self,
        national_id_type_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/national_id_types/:national_id_type_id".to_string();
        path = path.replace(":national_id_type_id", national_id_type_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/national_id_type/get
    pub async fn get_open_apis_corehr_v1_national_id_types_by_national_id_type_id(
        &self,
        national_id_type_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/national_id_types/:national_id_type_id".to_string();
        path = path.replace(":national_id_type_id", national_id_type_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/national_id_type/list
    pub async fn get_open_apis_corehr_v1_national_id_types(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/national_id_types".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
