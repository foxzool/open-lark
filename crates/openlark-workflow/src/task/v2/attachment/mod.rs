use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Attachment {
    service: Arc<WorkflowService>,
}

impl Attachment {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/task-v2/attachment/upload
    pub async fn post_open_apis_task_v2_attachments_upload(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/attachments/upload".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/attachment/list
    pub async fn get_open_apis_task_v2_attachments(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/attachments".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/attachment/get
    pub async fn get_open_apis_task_v2_attachments_by_attachment_guid(&self, attachment_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/attachments/:attachment_guid".to_string();
        path = path.replace(":attachment_guid", attachment_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/attachment/delete
    pub async fn delete_open_apis_task_v2_attachments_by_attachment_guid(&self, attachment_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/attachments/:attachment_guid".to_string();
        path = path.replace(":attachment_guid", attachment_guid.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
