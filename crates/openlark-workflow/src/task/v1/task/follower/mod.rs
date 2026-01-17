use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct TaskFollower {
    service: Arc<WorkflowService>,
}

impl TaskFollower {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task-follower/create
    pub async fn post_open_apis_task_v1_tasks_by_task_id_followers(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/followers".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task-follower/delete
    pub async fn delete_open_apis_task_v1_tasks_by_task_id_followers_by_follower_id(&self, task_id: impl AsRef<str>, follower_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/followers/:follower_id".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        path = path.replace(":follower_id", follower_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task-follower/list
    pub async fn get_open_apis_task_v1_tasks_by_task_id_followers(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/followers".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
