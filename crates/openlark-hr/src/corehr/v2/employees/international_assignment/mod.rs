use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmployeesInternationalAssignment {
    service: Arc<HrService>,
}

impl EmployeesInternationalAssignment {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/employee/job_data/employees-international_assignment/create
    pub async fn post_open_apis_corehr_v2_employees_international_assignments(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/corehr/v2/employees/international_assignments".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/employee/job_data/employees-international_assignment/patch
    pub async fn patch_open_apis_corehr_v2_employees_international_assignments_by_international_assignment_id(
        &self,
        international_assignment_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/corehr/v2/employees/international_assignments/:international_assignment_id"
                .to_string();
        path = path.replace(
            ":international_assignment_id",
            international_assignment_id.as_ref(),
        );
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/employee/job_data/employees-international_assignment/list
    pub async fn get_open_apis_corehr_v2_employees_international_assignments(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let path = "/open-apis/corehr/v2/employees/international_assignments".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/corehr-v1/employee/job_data/employees-international_assignment/delete
    pub async fn delete_open_apis_corehr_v2_employees_international_assignments_by_international_assignment_id(
        &self,
        international_assignment_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/corehr/v2/employees/international_assignments/:international_assignment_id"
                .to_string();
        path = path.replace(
            ":international_assignment_id",
            international_assignment_id.as_ref(),
        );
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
