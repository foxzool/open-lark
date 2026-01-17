pub mod comment;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Instance {
    service: Arc<WorkflowService>,
}

impl Instance {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance/create
    pub async fn post_open_apis_approval_v4_instances(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance/cancel
    pub async fn post_open_apis_approval_v4_instances_cancel(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/cancel".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance/cc
    pub async fn post_open_apis_approval_v4_instances_cc(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/cc".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance/approval-preview
    pub async fn post_open_apis_approval_v4_instances_preview(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/preview".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance/get
    pub async fn get_open_apis_approval_v4_instances_by_instance_id(&self, instance_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/:instance_id".to_string();
        path = path.replace(":instance_id", instance_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/instance/list
    pub async fn get_open_apis_approval_v4_instances(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/task/specified_rollback
    pub async fn post_open_apis_approval_v4_instances_specified_rollback(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/specified_rollback".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/task/approval-task-addsign
    pub async fn post_open_apis_approval_v4_instances_add_sign(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/add_sign".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/approval-search/query-2
    pub async fn post_open_apis_approval_v4_instances_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/approval-search/search_cc
    pub async fn post_open_apis_approval_v4_instances_search_cc(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/approval/v4/instances/search_cc".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
