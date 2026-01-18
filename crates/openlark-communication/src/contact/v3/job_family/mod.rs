//! 职级序列管理
//!
//! 提供企业职级序列（如技术序列、管理序列等）的管理功能。

use crate::service::CommunicationService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct JobFamily {
    service: Arc<CommunicationService>,
}

impl JobFamily {
    pub fn new(service: Arc<CommunicationService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/job_family/create
    pub async fn post_open_apis_contact_v3_job_families(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/job_families".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/job_family/update
    pub async fn put_open_apis_contact_v3_job_families_by_job_family_id(
        &self,
        job_family_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/job_families/:job_family_id".to_string();
        path = path.replace(":job_family_id", job_family_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/job_family/get
    pub async fn get_open_apis_contact_v3_job_families_by_job_family_id(
        &self,
        job_family_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/job_families/:job_family_id".to_string();
        path = path.replace(":job_family_id", job_family_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/job_family/list
    pub async fn get_open_apis_contact_v3_job_families(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/job_families".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/job_family/delete
    pub async fn delete_open_apis_contact_v3_job_families_by_job_family_id(
        &self,
        job_family_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/job_families/:job_family_id".to_string();
        path = path.replace(":job_family_id", job_family_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
