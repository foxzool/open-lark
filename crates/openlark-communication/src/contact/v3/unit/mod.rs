//! 单位管理
//!
//! 提供企业单位（如子公司、分公司等）的管理功能，支持单位与部门的关联管理。

use crate::service::CommunicationService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Unit {
    service: Arc<CommunicationService>,
}

impl Unit {
    pub fn new(service: Arc<CommunicationService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/unit/create
    pub async fn post_open_apis_contact_v3_unit(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/unit".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/unit/patch
    pub async fn patch_open_apis_contact_v3_unit_by_unit_id(
        &self,
        unit_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/unit/:unit_id".to_string();
        path = path.replace(":unit_id", unit_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/unit/bind_department
    pub async fn post_open_apis_contact_v3_unit_bind_department(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/unit/bind_department".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/unit/unbind_department
    pub async fn post_open_apis_contact_v3_unit_unbind_department(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/unit/unbind_department".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/unit/list_department
    pub async fn get_open_apis_contact_v3_unit_list_department(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/unit/list_department".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/unit/get
    pub async fn get_open_apis_contact_v3_unit_by_unit_id(
        &self,
        unit_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/unit/:unit_id".to_string();
        path = path.replace(":unit_id", unit_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/unit/list
    pub async fn get_open_apis_contact_v3_unit(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/unit".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/unit/delete
    pub async fn delete_open_apis_contact_v3_unit_by_unit_id(
        &self,
        unit_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/unit/:unit_id".to_string();
        path = path.replace(":unit_id", unit_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
