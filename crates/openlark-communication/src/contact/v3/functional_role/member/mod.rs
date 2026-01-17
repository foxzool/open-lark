use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct FunctionalRoleMember {
    service: Arc<CommunicationService>,
}

impl FunctionalRoleMember {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/batch_create
    pub async fn post_open_apis_contact_v3_functional_roles_by_role_id_members_batch_create(&self, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/functional_roles/:role_id/members/batch_create".to_string();
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/scopes
    pub async fn patch_open_apis_contact_v3_functional_roles_by_role_id_members_scopes(&self, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/functional_roles/:role_id/members/scopes".to_string();
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/get
    pub async fn get_open_apis_contact_v3_functional_roles_by_role_id_members_by_member_id(&self, role_id: impl AsRef<str>, member_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/functional_roles/:role_id/members/:member_id".to_string();
        path = path.replace(":role_id", role_id.as_ref());
        path = path.replace(":member_id", member_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/list
    pub async fn get_open_apis_contact_v3_functional_roles_by_role_id_members(&self, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/functional_roles/:role_id/members".to_string();
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/batch_delete
    pub async fn patch_open_apis_contact_v3_functional_roles_by_role_id_members_batch_delete(&self, role_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/contact/v3/functional_roles/:role_id/members/batch_delete".to_string();
        path = path.replace(":role_id", role_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
