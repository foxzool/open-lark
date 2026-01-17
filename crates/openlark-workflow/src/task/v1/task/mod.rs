pub mod collaborator;
pub mod comment;
pub mod follower;
pub mod reminder;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Task {
    service: Arc<WorkflowService>,
}

impl Task {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task/create
    pub async fn post_open_apis_task_v1_tasks(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task/delete
    pub async fn delete_open_apis_task_v1_tasks_by_task_id(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task/patch
    pub async fn patch_open_apis_task_v1_tasks_by_task_id(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task/complete
    pub async fn post_open_apis_task_v1_tasks_by_task_id_complete(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/complete".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task/uncomplete
    pub async fn post_open_apis_task_v1_tasks_by_task_id_uncomplete(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/uncomplete".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task/get
    pub async fn get_open_apis_task_v1_tasks_by_task_id(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task/list
    pub async fn get_open_apis_task_v1_tasks(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task-follower/batch_delete_follower
    pub async fn post_open_apis_task_v1_tasks_by_task_id_batch_delete_follower(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/batch_delete_follower".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task-collaborator/batch_delete_collaborator
    pub async fn post_open_apis_task_v1_tasks_by_task_id_batch_delete_collaborator(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/batch_delete_collaborator".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
