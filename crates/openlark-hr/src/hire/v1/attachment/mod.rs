use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Attachment {
    service: Arc<HrService>,
}

impl Attachment {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/attachment/create_attachment
    pub async fn post_open_apis_hire_v1_attachments(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/attachments".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/attachment/get
    pub async fn get_open_apis_hire_v1_attachments_by_attachment_id(&self, attachment_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/attachments/:attachment_id".to_string();
        path = path.replace(":attachment_id", attachment_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/attachment/preview
    pub async fn get_open_apis_hire_v1_attachments_by_attachment_id_preview(&self, attachment_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/attachments/:attachment_id/preview".to_string();
        path = path.replace(":attachment_id", attachment_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
