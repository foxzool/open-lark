use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Default {
    service: Arc<WorkflowService>,
}

impl Default {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/file/upload-files
    pub async fn post_approval_openapi_v2_file_upload(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/file/upload".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/external_approval/quick-approval-callback
    pub async fn post_approval_openapi_v2_external_instanceoperate(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/external/instanceOperate".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/message/send-bot-messages
    pub async fn post_approval_openapi_v1_message_send(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v1/message/send".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/message/update-bot-messages
    pub async fn post_approval_openapi_v1_message_update(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v1/message/update".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/approval-v4/approval-search/search-approval-id-(dedicated)
    pub async fn post_approval_openapi_v1_id_get(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v1/id/get".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/subscribe-to-an-approvals-event-
    pub async fn post_approval_openapi_v2_subscription_subscribe(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/subscription/subscribe".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/cancel-a-subscription-to-an-approvals-event
    pub async fn post_approval_openapi_v2_subscription_unsubscribe(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/subscription/unsubscribe".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/obtain-single-approval-form
    pub async fn post_approval_openapi_v2_approval_get(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/approval/get".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/create-approval-instance
    pub async fn post_approval_openapi_v2_instance_create(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/create".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/obtain-single-approval-instance-details
    pub async fn post_approval_openapi_v2_instance_get(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/get".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/batch-obtain-approval-instance-ids
    pub async fn post_approval_openapi_v2_instance_list(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/list".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/cc-instance
    pub async fn post_approval_openapi_v2_instance_cc(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/cc".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/approval-instance-cancel
    pub async fn post_approval_openapi_v2_instance_cancel(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/cancel".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/approval-task-approve
    pub async fn post_approval_openapi_v2_instance_approve(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/approve".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/approval-task-reject
    pub async fn post_approval_openapi_v2_instance_reject(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/reject".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-native-approval/approval-task-transfer
    pub async fn post_approval_openapi_v2_instance_transfer(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/transfer".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/third-party-approval-integration/external-approval-create
    pub async fn post_approval_openapi_v3_external_approval_create(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v3/external/approval/create".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/third-party-approval-integration/external-approval-instance-create
    pub async fn post_approval_openapi_v2_external_instance_create(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/external/instance/create".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/third-party-approval-integration/external-approval-instance-check
    pub async fn post_approval_openapi_v3_external_instance_check(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v3/external/instance/check".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/third-party-approval-integration/external_status
    pub async fn post_approval_openapi_v2_external_list(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/external/list".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/feishu-store-app-integration/create-an-approval-definition
    pub async fn post_approval_openapi_v2_approval_create(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/approval/create".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/approval-search/instance-list-query
    pub async fn post_approval_openapi_v2_instance_search(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/instance/search".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/approval-search/cc-list-query
    pub async fn post_approval_openapi_v2_cc_search(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/cc/search".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/historic-version/approval/v2/approval-search/task-list-query
    pub async fn post_approval_openapi_v2_task_search(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/approval/openapi/v2/task/search".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
