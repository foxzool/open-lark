//! 用户组成员管理
//!
//! 提供用户组成员的增删改查功能。

use crate::service::CommunicationService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct GroupMember {
    service: Arc<CommunicationService>,
}

impl GroupMember {
    pub fn new(service: Arc<CommunicationService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group-member/add
    pub async fn post_open_apis_contact_v3_group_by_group_id_member_add(
        &self,
        group_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/:group_id/member/add".to_string();
        path = path.replace(":group_id", group_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group-member/batch_add
    pub async fn post_open_apis_contact_v3_group_by_group_id_member_batch_add(
        &self,
        group_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/:group_id/member/batch_add".to_string();
        path = path.replace(":group_id", group_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group-member/simplelist
    pub async fn get_open_apis_contact_v3_group_by_group_id_member_simplelist(
        &self,
        group_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/:group_id/member/simplelist".to_string();
        path = path.replace(":group_id", group_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group-member/remove
    pub async fn post_open_apis_contact_v3_group_by_group_id_member_remove(
        &self,
        group_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/:group_id/member/remove".to_string();
        path = path.replace(":group_id", group_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group-member/batch_remove
    pub async fn post_open_apis_contact_v3_group_by_group_id_member_batch_remove(
        &self,
        group_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/:group_id/member/batch_remove".to_string();
        path = path.replace(":group_id", group_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
