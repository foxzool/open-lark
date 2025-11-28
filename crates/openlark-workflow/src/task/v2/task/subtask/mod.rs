use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct TaskSubtask {
    service: Arc<WorkflowService>,
}

impl TaskSubtask {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task-subtask/create
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_subtasks(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/subtasks".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task-subtask/list
    pub async fn get_open_apis_task_v2_tasks_by_task_guid_subtasks(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/subtasks".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
