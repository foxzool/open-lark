use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AppSkill {
    service: Arc<CommunicationService>,
}

impl AppSkill {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/app-skill/start
    pub async fn post_open_apis_aily_v1_apps_by_app_id_skills_by_skill_id_start(&self, app_id: impl AsRef<str>, skill_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/apps/:app_id/skills/:skill_id/start".to_string();
        path = path.replace(":app_id", app_id.as_ref());
        path = path.replace(":skill_id", skill_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/app-skill/get
    pub async fn get_open_apis_aily_v1_apps_by_app_id_skills_by_skill_id(&self, app_id: impl AsRef<str>, skill_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/apps/:app_id/skills/:skill_id".to_string();
        path = path.replace(":app_id", app_id.as_ref());
        path = path.replace(":skill_id", skill_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/app-skill/list
    pub async fn get_open_apis_aily_v1_apps_by_app_id_skills(&self, app_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/apps/:app_id/skills".to_string();
        path = path.replace(":app_id", app_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
