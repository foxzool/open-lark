use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApprovalGroups {
    service: Arc<HrService>,
}

impl ApprovalGroups {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/approval_groups/open_query_position_change_list_by_ids
    pub async fn post_open_apis_corehr_v2_approval_groups_open_query_position_change_list_by_ids(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/corehr/v2/approval_groups/open_query_position_change_list_by_ids"
            .to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/approval_groups/get
    pub async fn get_open_apis_corehr_v2_approval_groups_by_process_id(
        &self,
        process_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v2/approval_groups/:process_id".to_string();
        path = path.replace(":process_id", process_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/approval_groups/open_query_department_change_list_by_ids
    pub async fn post_open_apis_corehr_v2_approval_groups_open_query_department_change_list_by_ids(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/corehr/v2/approval_groups/open_query_department_change_list_by_ids"
            .to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/approval_groups/open_query_job_change_list_by_ids
    pub async fn post_open_apis_corehr_v2_approval_groups_open_query_job_change_list_by_ids(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path =
            "/open-apis/corehr/v2/approval_groups/open_query_job_change_list_by_ids".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
