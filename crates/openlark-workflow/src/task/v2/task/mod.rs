pub mod subtask;

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

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/create
    pub async fn post_open_apis_task_v2_tasks(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/patch
    pub async fn patch_open_apis_task_v2_tasks_by_task_guid(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/get
    pub async fn get_open_apis_task_v2_tasks_by_task_guid(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/delete
    pub async fn delete_open_apis_task_v2_tasks_by_task_guid(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/add_members
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_add_members(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/add_members".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/remove_members
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_remove_members(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/remove_members".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/list
    pub async fn get_open_apis_task_v2_tasks(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/tasklists
    pub async fn get_open_apis_task_v2_tasks_by_task_guid_tasklists(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/tasklists".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/add_tasklist
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_add_tasklist(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/add_tasklist".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/remove_tasklist
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_remove_tasklist(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/remove_tasklist".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/add_reminders
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_add_reminders(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/add_reminders".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/remove_reminders
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_remove_reminders(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/remove_reminders".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/add_dependencies
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_add_dependencies(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/add_dependencies".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/task/remove_dependencies
    pub async fn post_open_apis_task_v2_tasks_by_task_guid_remove_dependencies(&self, task_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasks/:task_guid/remove_dependencies".to_string();
        path = path.replace(":task_guid", task_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
