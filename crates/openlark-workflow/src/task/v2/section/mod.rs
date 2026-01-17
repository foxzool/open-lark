use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Section {
    service: Arc<WorkflowService>,
}

impl Section {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/task-v2/section/create
    pub async fn post_open_apis_task_v2_sections(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/sections".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/section/get
    pub async fn get_open_apis_task_v2_sections_by_section_guid(&self, section_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/sections/:section_guid".to_string();
        path = path.replace(":section_guid", section_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/section/patch
    pub async fn patch_open_apis_task_v2_sections_by_section_guid(&self, section_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/sections/:section_guid".to_string();
        path = path.replace(":section_guid", section_guid.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/section/delete
    pub async fn delete_open_apis_task_v2_sections_by_section_guid(&self, section_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/sections/:section_guid".to_string();
        path = path.replace(":section_guid", section_guid.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/section/list
    pub async fn get_open_apis_task_v2_sections(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/sections".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/section/tasks
    pub async fn get_open_apis_task_v2_sections_by_section_guid_tasks(&self, section_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/sections/:section_guid/tasks".to_string();
        path = path.replace(":section_guid", section_guid.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
