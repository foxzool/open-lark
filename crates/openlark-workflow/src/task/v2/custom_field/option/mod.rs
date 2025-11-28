use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct CustomFieldOption {
    service: Arc<WorkflowService>,
}

impl CustomFieldOption {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/task-v2/custom_field-option/create
    pub async fn post_open_apis_task_v2_custom_fields_by_custom_field_guid_options(&self, custom_field_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/custom_fields/:custom_field_guid/options".to_string();
        path = path.replace(":custom_field_guid", custom_field_guid.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/task-v2/custom_field-option/patch
    pub async fn patch_open_apis_task_v2_custom_fields_by_custom_field_guid_options_by_option_guid(&self, custom_field_guid: impl AsRef<str>, option_guid: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/task/v2/custom_fields/:custom_field_guid/options/:option_guid".to_string();
        path = path.replace(":custom_field_guid", custom_field_guid.as_ref());
        path = path.replace(":option_guid", option_guid.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
