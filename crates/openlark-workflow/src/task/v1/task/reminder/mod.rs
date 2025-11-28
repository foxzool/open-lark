use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct TaskReminder {
    service: Arc<WorkflowService>,
}

impl TaskReminder {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task-reminder/create
    pub async fn post_open_apis_task_v1_tasks_by_task_id_reminders(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/reminders".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task-reminder/delete
    pub async fn delete_open_apis_task_v1_tasks_by_task_id_reminders_by_reminder_id(&self, task_id: impl AsRef<str>, reminder_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/reminders/:reminder_id".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        path = path.replace(":reminder_id", reminder_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/task-v1/task-reminder/list
    pub async fn get_open_apis_task_v1_tasks_by_task_id_reminders(&self, task_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v1/tasks/:task_id/reminders".to_string();
        path = path.replace(":task_id", task_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
