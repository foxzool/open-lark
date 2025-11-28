use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct CostAllocationReport {
    service: Arc<HrService>,
}

impl CostAllocationReport {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/payroll-v1/cost_allocation_report/list
    pub async fn get_open_apis_payroll_v1_cost_allocation_reports(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/payroll/v1/cost_allocation_reports".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
