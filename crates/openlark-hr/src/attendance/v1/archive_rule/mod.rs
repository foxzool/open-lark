use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::HrService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ArchiveRule {
    service: Arc<HrService>,
}

impl ArchiveRule {
    pub fn new(service: Arc<HrService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/attendance-v1/archive_rule/user_stats_fields_query
    pub async fn post_open_apis_attendance_v1_archive_rule_user_stats_fields_query(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/archive_rule/user_stats_fields_query".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/attendance-v1/archive_rule/upload_report
    pub async fn post_open_apis_attendance_v1_archive_rule_upload_report(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/archive_rule/upload_report".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/attendance-v1/archive_rule/del_report
    pub async fn post_open_apis_attendance_v1_archive_rule_del_report(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/archive_rule/del_report".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/attendance-v1/archive_rule/list
    pub async fn get_open_apis_attendance_v1_archive_rule(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/attendance/v1/archive_rule".to_string();
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
