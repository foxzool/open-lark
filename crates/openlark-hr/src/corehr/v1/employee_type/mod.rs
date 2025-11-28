use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct EmployeeType {
    service: Arc<HrService>,
}

impl EmployeeType {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/employee_type/create
    pub async fn post_open_apis_corehr_v1_employee_types(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/employee_types".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/employee_type/delete
    pub async fn delete_open_apis_corehr_v1_employee_types_by_employee_type_id(&self, employee_type_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/employee_types/:employee_type_id".to_string();
        path = path.replace(":employee_type_id", employee_type_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/employee_type/patch
    pub async fn patch_open_apis_corehr_v1_employee_types_by_employee_type_id(&self, employee_type_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/employee_types/:employee_type_id".to_string();
        path = path.replace(":employee_type_id", employee_type_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/employee_type/get
    pub async fn get_open_apis_corehr_v1_employee_types_by_employee_type_id(&self, employee_type_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/employee_types/:employee_type_id".to_string();
        path = path.replace(":employee_type_id", employee_type_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/employee_type/list
    pub async fn get_open_apis_corehr_v1_employee_types(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/corehr/v1/employee_types".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
