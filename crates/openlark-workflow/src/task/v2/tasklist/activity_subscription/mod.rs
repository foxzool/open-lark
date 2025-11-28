use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct TasklistActivitySubscription {
    service: Arc<WorkflowService>,
}

impl TasklistActivitySubscription {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/create
    pub async fn post_open_apis_task_v2_tasklists_by_tasklist_guid_activity_subscriptions(&self, tasklist_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/get
    pub async fn get_open_apis_task_v2_tasklists_by_tasklist_guid_activity_subscriptions_by_activity_subscription_guid(&self, tasklist_guid: impl AsRef<str>, activity_subscription_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        path = path.replace(":activity_subscription_guid", activity_subscription_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/list
    pub async fn get_open_apis_task_v2_tasklists_by_tasklist_guid_activity_subscriptions(&self, tasklist_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/patch
    pub async fn patch_open_apis_task_v2_tasklists_by_tasklist_guid_activity_subscriptions_by_activity_subscription_guid(&self, tasklist_guid: impl AsRef<str>, activity_subscription_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        path = path.replace(":activity_subscription_guid", activity_subscription_guid.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/delete
    pub async fn delete_open_apis_task_v2_tasklists_by_tasklist_guid_activity_subscriptions_by_activity_subscription_guid(&self, tasklist_guid: impl AsRef<str>, activity_subscription_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid".to_string();
        path = path.replace(":tasklist_guid", tasklist_guid.as_ref());
        path = path.replace(":activity_subscription_guid", activity_subscription_guid.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
