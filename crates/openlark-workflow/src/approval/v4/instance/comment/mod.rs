use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct InstanceComment {
    service: Arc<WorkflowService>,
}

impl InstanceComment {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance-comment/create
    pub async fn post_open_apis_approval_v4_instances_by_instance_id_comments(&self, instance_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/:instance_id/comments".to_string();
        path = path.replace(":instance_id", instance_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance-comment/delete
    pub async fn delete_open_apis_approval_v4_instances_by_instance_id_comments_by_comment_id(&self, instance_id: impl AsRef<str>, comment_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/:instance_id/comments/:comment_id".to_string();
        path = path.replace(":instance_id", instance_id.as_ref());
        path = path.replace(":comment_id", comment_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance-comment/remove
    pub async fn post_open_apis_approval_v4_instances_by_instance_id_comments_remove(&self, instance_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/:instance_id/comments/remove".to_string();
        path = path.replace(":instance_id", instance_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance-comment/list
    pub async fn get_open_apis_approval_v4_instances_by_instance_id_comments(&self, instance_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/:instance_id/comments".to_string();
        path = path.replace(":instance_id", instance_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
