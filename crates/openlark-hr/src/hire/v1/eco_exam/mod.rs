use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct EcoExam {
    service: Arc<HrService>,
}

impl EcoExam {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/eco_exam/login_info
    pub async fn post_open_apis_hire_v1_eco_exams_by_exam_id_login_info(
        &self,
        exam_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/eco_exams/:exam_id/login_info".to_string();
        path = path.replace(":exam_id", exam_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/eco_exam/update_result
    pub async fn post_open_apis_hire_v1_eco_exams_by_exam_id_update_result(
        &self,
        exam_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/eco_exams/:exam_id/update_result".to_string();
        path = path.replace(":exam_id", exam_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
