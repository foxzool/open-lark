pub mod member;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Group {
    service: Arc<CommunicationService>,
}

impl Group {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group/create
    pub async fn post_open_apis_contact_v3_group(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group/patch
    pub async fn patch_open_apis_contact_v3_group_by_group_id(&self, group_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/:group_id".to_string();
        path = path.replace(":group_id", group_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group/get
    pub async fn get_open_apis_contact_v3_group_by_group_id(&self, group_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/:group_id".to_string();
        path = path.replace(":group_id", group_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group/simplelist
    pub async fn get_open_apis_contact_v3_group_simplelist(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/simplelist".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group/member_belong
    pub async fn get_open_apis_contact_v3_group_member_belong(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/member_belong".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/group/delete
    pub async fn delete_open_apis_contact_v3_group_by_group_id(&self, group_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/group/:group_id".to_string();
        path = path.replace(":group_id", group_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
