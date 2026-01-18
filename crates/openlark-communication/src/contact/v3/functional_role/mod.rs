//! 功能角色管理
//!
//! 提供企业功能角色（如管理员、审计员等）的管理功能。

pub mod member;

use crate::service::CommunicationService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct FunctionalRole {
    service: Arc<CommunicationService>,
}

impl FunctionalRole {
    pub fn new(service: Arc<CommunicationService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/functional_role/create
    pub async fn post_open_apis_contact_v3_functional_roles(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/functional_roles".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/functional_role/update
    pub async fn put_open_apis_contact_v3_functional_roles_by_role_id(
        &self,
        role_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/functional_roles/:role_id".to_string();
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::PUT;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/functional_role/delete
    pub async fn delete_open_apis_contact_v3_functional_roles_by_role_id(
        &self,
        role_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/functional_roles/:role_id".to_string();
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
