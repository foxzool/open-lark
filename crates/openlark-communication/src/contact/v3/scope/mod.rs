//! 权限范围管理
//!
//! 提供应用权限范围的查询功能。

use crate::service::CommunicationService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Scope {
    service: Arc<CommunicationService>,
}

impl Scope {
    pub fn new(service: Arc<CommunicationService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/scope/list
    pub async fn get_open_apis_contact_v3_scopes(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/scopes".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
