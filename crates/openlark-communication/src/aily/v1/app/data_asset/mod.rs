use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct AppDataAsset {
    service: Arc<CommunicationService>,
}

impl AppDataAsset {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/upload_file
    pub async fn post_open_apis_aily_v1_apps_by_app_id_data_assets_upload_file(&self, app_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/apps/:app_id/data_assets/upload_file".to_string();
        path = path.replace(":app_id", app_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/create
    pub async fn post_open_apis_aily_v1_apps_by_app_id_data_assets(&self, app_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/apps/:app_id/data_assets".to_string();
        path = path.replace(":app_id", app_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/get
    pub async fn get_open_apis_aily_v1_apps_by_app_id_data_assets_by_data_asset_id(&self, app_id: impl AsRef<str>, data_asset_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/apps/:app_id/data_assets/:data_asset_id".to_string();
        path = path.replace(":app_id", app_id.as_ref());
        path = path.replace(":data_asset_id", data_asset_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/delete
    pub async fn delete_open_apis_aily_v1_apps_by_app_id_data_assets_by_data_asset_id(&self, app_id: impl AsRef<str>, data_asset_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/apps/:app_id/data_assets/:data_asset_id".to_string();
        path = path.replace(":app_id", app_id.as_ref());
        path = path.replace(":data_asset_id", data_asset_id.as_ref());
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/list
    pub async fn get_open_apis_aily_v1_apps_by_app_id_data_assets(&self, app_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/aily/v1/apps/:app_id/data_assets".to_string();
        path = path.replace(":app_id", app_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
