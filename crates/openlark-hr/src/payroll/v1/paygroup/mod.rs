use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct Paygroup {
    service: Arc<HrService>,
}

impl Paygroup {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/paygroup/list
    pub async fn get_open_apis_payroll_v1_paygroups(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/payroll/v1/paygroups".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
