pub mod activity_subscription;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Tasklist {
    service: Arc<WorkflowService>,
}

impl Tasklist {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist/create
    pub async fn post_open_apis_task_v2_tasklists(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist/get
    pub async fn get_open_apis_task_v2_tasklists_by_tasklist_guid(&self, tasklist_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist/patch
    pub async fn patch_open_apis_task_v2_tasklists_by_tasklist_guid(&self, tasklist_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist/delete
    pub async fn delete_open_apis_task_v2_tasklists_by_tasklist_guid(&self, tasklist_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist/add_members
    pub async fn post_open_apis_task_v2_tasklists_by_tasklist_guid_add_members(&self, tasklist_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid/add_members".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist/remove_members
    pub async fn post_open_apis_task_v2_tasklists_by_tasklist_guid_remove_members(&self, tasklist_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid/remove_members".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist/tasks
    pub async fn get_open_apis_task_v2_tasklists_by_tasklist_guid_tasks(&self, tasklist_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid/tasks".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist/list
    pub async fn get_open_apis_task_v2_tasklists(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
